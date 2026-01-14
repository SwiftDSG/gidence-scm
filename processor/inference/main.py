"""
Evidence JSON Format:
{
    "camera_id": "cam_1",
    "frame_id": 123,
    "timestamp": 1704672345123 # epoch time in milliseconds
    "person": [
        {
            "person_id": "person_000",
            "bbox": [100, 150, 200, 350],
            "confidence": 0.95,
            "part": [
                {
                    "label": "hand",
                    "bbox": [120, 280, 160, 320],
                    "confidence": 0.87
                }
            ],
            "equipment": [
                {
                    "label": "hardhat",
                    "bbox": [108, 150, 192, 205],
                    "confidence": 0.89
                }
            ],
            "violation": [
                "missing_gloves"
            ]
        }
        ...
    ]
}

List of Violations:
    - missing_hardhat (head + no hardhat)
    - missing_gloves (hand + no gloves)
    - missing_shoes (foot + no shoes)
    - missing_facemask (face + no facemask)
    - missing_earmuffs (ear + no earmuffs)
    - improperly_worn_gloves (hand + gloves)
    - improperly_worn_shoes (foot + shoes)
    - improperly_worn_facemask (face + facemask)
    - improperly_worn_earmuffs (ear + earmuffs)
"""

# region imports
# Standard library imports

# Third-party imports
import gi
import time
import hailo
import json

from pathlib import Path

gi.require_version("Gst", "1.0")

# Local application-specific imports
from gi.repository import Gst

from inference.pipeline import SCMPipeline
from inference.udp import UDPSender

from inference.core.common.buffer_utils import get_caps_from_pad, get_numpy_from_buffer
from inference.core.common.hailo_logger import get_logger
from inference.core.gstreamer.gstreamer_app import app_callback_class

# SCM modules
from inference.association import (
    assign_detections_to_persons,
    check_compliance_all_persons,
)

logger = get_logger(__name__)

# endregion imports

# -----------------------------------------------------------------------------------------------
# User-defined configuration class
# -----------------------------------------------------------------------------------------------
class SCMConfig:
    """
    SCM System Configuration.

    Loads and provides access to:
    - Model path
    - Camera configurations
    - UDP settings
    """

    def __init__(self, path: str = None):
        """
        Load configuration from JSON file.

        Args:
            path: Path to config.json file
                If None, uses default: processor/config.json
        """
        if path is None:
            # Default path: processor/config.json
            script_dir = Path(__file__).parent.parent
            path = script_dir / "config.json"

        path = Path(path)

        if not path.exists():
            logger.error(f"Config file not found: {path}")
            raise FileNotFoundError(f"Config file not found: {path}")

        logger.info(f"Loading config from: {path}")

        with open(path, 'r') as f:
            config = json.load(f)
            self.model = config.get("model", "yolov8n.hef")
            self.cameras = config.get("camera", [])
            self.udp = config.get("udp", None)

        logger.info("Configuration loaded successfully")


# -----------------------------------------------------------------------------------------------
# User-defined class to be used in the callback function: Inheritance from the app_callback_class
# -----------------------------------------------------------------------------------------------
class SCM(app_callback_class):
    def __init__(self):
        super().__init__()

        # Load SCM configuration
        self.config = SCMConfig()

        # Initialize UDP sender
        if self.config.udp is not None:
            self.udp = UDPSender(
                host=self.config.udp.get("host", "127.0.0.1"),
                port=self.config.udp.get("port", 8888)
            )
        else:
            self.udp = None


# -----------------------------------------------------------------------------------------------
# User-defined callback function
# -----------------------------------------------------------------------------------------------
def callback(element, buffer, data):
    """
    SCM Callback for PPE compliance checking.

    Uses body part visibility detection:
    - If body part detected (e.g., 'hand') → check for PPE (e.g., 'gloves')
    - If PPE detected → compliant (body part covered)
    - If body part detected but no PPE → VIOLATION
    - If neither → body part not visible → skip check
    """
    frame_idx = data.get_count()
    # logger.debug(f"[{data.camera_id}] Processing frame {frame_idx}")

    try:
        # Get the GStreamer buffer
        if buffer is None:
            logger.warning(f"[{data.camera_id}] Received None buffer at frame={frame_idx}")
            return

        # Get frame information (optional, for visualization)
        pad = element.get_static_pad("src")
        format, width, height = get_caps_from_pad(pad)

        # Extract frame data (for debugging/visualization if needed)
        frame = None
        if data.use_frame and format and width and height:
            frame = get_numpy_from_buffer(buffer, format, width, height)

        # Get millisecond timestamp
        timestamp = buffer.pts // Gst.MSECOND

        # Get detections from Hailo buffer
        roi = hailo.get_roi_from_buffer(buffer)
        detections = roi.get_objects_typed(hailo.HAILO_DETECTION)
        camera_id = roi.get_stream_id()

        # Separate persons from other detections (body parts + PPE)
        persons = []
        others = []

        for detection in detections:
            label = detection.get_label()
            bbox = detection.get_bbox()
            confidence = detection.get_confidence()

            # Filter low-confidence detections
            if confidence < 0.3:
                continue

            det = {
                "label": label,
                "bbox": bbox,
                "confidence": confidence,
            }

            if label == "person":
                track = detection.get_objects_typed(hailo.HAILO_UNIQUE_ID)
                if len(track) == 1:
                    track_id = track[0].get_id()
                else:
                    continue  # Skip if no track ID

                det["person_id"] = f"person_{track_id:03d}"
                persons.append(det)
            else:
                others.append(det)

        # Skip if no persons detected
        if len(persons) == 0:
            logger.debug(f"[{camera_id}] No persons detected in frame {frame_idx}")
            return

        # Assign body parts and PPE to each person
        person_assignments = assign_detections_to_persons(persons, others)

        # Check compliance for all persons
        persons = check_compliance_all_persons(person_assignments)

        # Log results
        violating = False
        for person in persons:
            if len(person.get("violation", [])) > 0:
                violating = True
                break
        
        if violating:
            if data.udp is None:
                # (TODO) Log violation locally
                logger.info(f"[{camera_id}] Violation detected in frame {frame_idx}, but UDP not configured.")
                for person in persons:
                    string_to_print = f" Person ID: {person['person_id']}"

                    # Log bbox and violations
                    string_to_print += f", BBox: {person['bbox']}"
                    string_to_print += f", Violations: {person['violation']}"

                    # Log
                    logger.info(string_to_print)
            else:
                # (TODO) Prepare UDP message
                # success = data.udp.send(
                #     camera_id=camera_id,
                #     frame_id=f"frame_{frame_idx:06d}",
                #     timestamp=timestamp,
                #     person=persons
                # )
                # if success:
                #     logger.info(f"[{camera_id}] Sent violation for frame {frame_idx} via UDP")
                # else:
                #     logger.error(f"[{camera_id}] Failed to send violation for frame {frame_idx} via UDP")
                pass
            
            # (TODO) save frame as evidence image

        return

    except Exception as e:
        logger.error(f"Error in callback: {e}", exc_info=True)
        return Gst.PadProbeReturn.OK


def main():
    logger.info("Starting Detection Simple App.")

    # Initialize SCM data
    data = SCM()

    # Create and run SCM pipeline
    app = SCMPipeline(callback, data, data.config["cameras"], model=data.config["model"])
    app.run()

if __name__ == "__main__":
    main()
