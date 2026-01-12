# region imports
# Standard library imports

# Third-party imports
import gi

gi.require_version("Gst", "1.0")

# Local application-specific imports
import hailo
from gi.repository import Gst

from inference.pipeline import (
    GStreamerDetectionSimpleApp,
)

from inference.core.common.buffer_utils import get_caps_from_pad, get_numpy_from_buffer
from inference.core.common.hailo_logger import get_logger
from inference.core.gstreamer.gstreamer_app import app_callback_class

hailo_logger = get_logger(__name__)

# endregion imports

# -----------------------------------------------------------------------------------------------
# User-defined class to be used in the callback function: Inheritance from the app_callback_class
# -----------------------------------------------------------------------------------------------
class SCMCallback(app_callback_class):
    def __init__(self):
        super().__init__()

# -----------------------------------------------------------------------------------------------
# User-defined callback function
# -----------------------------------------------------------------------------------------------
def scm_callback(element, buffer, user_data):
    # Note: Frame counting is handled automatically by the framework wrapper
    frame_idx = user_data.get_count()
    hailo_logger.debug("Processing frame %s", frame_idx)
    string_to_print = f"Frame count: {user_data.get_count()}\n"
    
    try:
        # Get the GStreamer buffer
        if buffer is None:
            hailo_logger.warning("Received None buffer at frame=%s", user_data.get_count())
            return
            
        # Get frame information
        pad = element.get_static_pad("src")
        format, width, height = get_caps_from_pad(pad)
        if format is None or width is None or height is None:
            return Gst.PadProbeReturn.OK
        
        # Extract frame data (for debugging/visualization)
        frame = None
        if user_data.use_frame and format and width and height:
            frame = get_numpy_from_buffer(buffer, format, width, height)
            
        # Get detections from Hailo buffer
        roi = hailo.get_roi_from_buffer(buffer)
        detections = roi.get_objects_typed(hailo.HAILO_DETECTION)

        persons = []
        others = []

        # First loop: Create a map of persons, body parts, and PPE detections
        for detection in detections:
            label = detection.get_label()
            bbox = detection.get_bbox()
            confidence = detection.get_confidence()

            if confidence < 0.3:
                continue

            det = {
                "label": label,
                "bbox": bbox,
                "confidence": confidence,
            }

            if label == "person":
                persons.append(det)
            else:
                others.append(det)


        # Second loop: Analyze relationships (e.g., PPE on persons)
        for person in persons:
            person_bbox = person["bbox"]
            person_conf = person["confidence"]

            person_xmin = person_bbox.xmin()
            person_ymin = person_bbox.ymin()
            person_xmax = person_bbox.xmax()
            person_ymax = person_bbox.ymax()
            hailo_logger.debug(f"Person detected with bbox: x1={person_xmin}, y1={person_ymin}, x2={person_xmax}, y2={person_ymax}")
            
            # Loop for other detections to see if they belong to this person
            for other in others:
                other_bbox = other["bbox"]
                other_conf = other["confidence"]
                other_label = other["label"]

                # Simple IoU check or containment check can be implemented here
                # For simplicity, make the person's bbox 10% larger for containment check
                expanded_person_bbox = [
                    person_bbox[0] - 0.05 * (person_bbox[2] - person_bbox[0]),
                    person_bbox[1] - 0.05 * (person_bbox[3] - person_bbox[1]),
                    person_bbox[2] + 0.05 * (person_bbox[2] - person_bbox[0]),
                    person_bbox[3] + 0.05 * (person_bbox[3] - person_bbox[1]),
                ]
                if (
                    other_bbox[0] >= expanded_person_bbox[0]
                    and other_bbox[1] >= expanded_person_bbox[1]
                    and other_bbox[2] <= expanded_person_bbox[2]
                    and other_bbox[3] <= expanded_person_bbox[3]
                ):
                    string_to_print += f"Person (conf: {person_conf:.2f}) has {other_label} (conf: {other_conf:.2f})\n"

                # TODO: Rules for determining if PPE is correctly worn can be added here
            
        print(string_to_print)
        return
            
    except Exception as e:
        hailo_logger.error(f"Error in callback: {e}")
        return Gst.PadProbeReturn.OK


def main():
    hailo_logger.info("Starting Detection Simple App.")
    user_data = SCMCallback()
    app = GStreamerDetectionSimpleApp(scm_callback, user_data)
    app.run()

if __name__ == "__main__":
    main()
