# region imports
# Standard library imports
import setproctitle
import numpy as np

import gi
gi.require_version("Gst", "1.0")
from gi.repository import Gst

from inference.core.common.core import get_pipeline_parser, handle_list_models_flag, resolve_hef_path
from inference.core.common.buffer_utils import get_caps_from_pad, get_numpy_from_buffer
from inference.core.common.defines import (
    POSE_ESTIMATION_PIPELINE,
)

from inference.core.common.hailo_logger import get_logger
from inference.core.gstreamer.gstreamer_app import (
    GStreamerApp,
    app_callback_class,
)
from inference.core.gstreamer.gstreamer_helper_pipelines import (
    DISPLAY_PIPELINE,
    INFERENCE_PIPELINE,
    SOURCE_PIPELINE,
    USER_CALLBACK_PIPELINE,
)

hailo_logger = get_logger(__name__)

# endregion imports

# -----------------------------------------------------------------------------------------------
# SCM Pose Detection Pipeline (Stage 1)
# -----------------------------------------------------------------------------------------------

# COCO keypoint indices for body part derivation
KEYPOINT_INDICES = {
    'nose': 0,
    'left_eye': 1,
    'right_eye': 2,
    'left_ear': 3,
    'right_ear': 4,
    'left_shoulder': 5,
    'right_shoulder': 6,
    'left_elbow': 7,
    'right_elbow': 8,
    'left_wrist': 9,
    'right_wrist': 10,
    'left_hip': 11,
    'right_hip': 12,
    'left_knee': 13,
    'right_knee': 14,
    'left_ankle': 15,
    'right_ankle': 16
}

def derive_body_parts_from_keypoints(keypoints):
    """
    Derive body part bounding boxes from 17 COCO keypoints.

    Args:
        keypoints: List of 17 tuples (x, y, confidence)

    Returns:
        dict: {
            'head': (x, y, w, h),
            'torso': (x, y, w, h),
            'hands': (x, y, w, h),
            'feet': (x, y, w, h)
        }
    """
    body_parts = {}

    # Head: from nose, eyes, ears (indices 0-4)
    head_points = [kp for kp in keypoints[0:5] if kp[2] > 0.5]
    if len(head_points) >= 2:
        xs = [p[0] for p in head_points]
        ys = [p[1] for p in head_points]
        margin = 30  # pixels
        body_parts['head'] = (
            min(xs) - margin,
            min(ys) - margin,
            max(xs) - min(xs) + 2*margin,
            max(ys) - min(ys) + 2*margin
        )

    # Torso: from shoulders, hips (indices 5,6,11,12)
    torso_points = [keypoints[i] for i in [5,6,11,12] if keypoints[i][2] > 0.5]
    if len(torso_points) >= 3:
        xs = [p[0] for p in torso_points]
        ys = [p[1] for p in torso_points]
        margin = 20
        body_parts['torso'] = (
            min(xs) - margin,
            min(ys) - margin,
            max(xs) - min(xs) + 2*margin,
            max(ys) - min(ys) + 2*margin
        )

    # Hands: from wrists (indices 9, 10)
    hand_points = [keypoints[i] for i in [9,10] if keypoints[i][2] > 0.5]
    if len(hand_points) >= 1:
        xs = [p[0] for p in hand_points]
        ys = [p[1] for p in hand_points]
        margin = 40
        body_parts['hands'] = (
            min(xs) - margin,
            min(ys) - margin,
            max(xs) - min(xs) + 2*margin,
            max(ys) - min(ys) + 2*margin
        )

    # Feet: from ankles (indices 15, 16)
    feet_points = [keypoints[i] for i in [15,16] if keypoints[i][2] > 0.5]
    if len(feet_points) >= 1:
        xs = [p[0] for p in feet_points]
        ys = [p[1] for p in feet_points]
        margin_h = 40
        margin_down = 60  # Extend down for shoes
        body_parts['feet'] = (
            min(xs) - margin_h,
            min(ys) - 10,
            max(xs) - min(xs) + 2*margin_h,
            max(ys) - min(ys) + margin_down
        )

    return body_parts


class SCMPoseDetectionApp(GStreamerApp):
    """
    SCM Pipeline v1: Pose Detection Only
    
    This pipeline implements Stage 1 of the SCM system:
    - Detects people using YOLOv8n-pose model
    - Extracts 17 COCO keypoints per person
    - Derives body part bounding boxes from keypoints
    - Prints results for verification
    """
    
    def __init__(self, app_callback, user_data, parser=None):
        if parser is None:
            parser = get_pipeline_parser()
        
        parser.add_argument(
            "--pose-threshold",
            type=float,
            default=0.5,
            help="Confidence threshold for pose keypoints",
        )
        
        # Handle --list-models flag before full initialization
        handle_list_models_flag(parser, POSE_ESTIMATION_PIPELINE)
        
        # Initialize parent
        super().__init__(parser, user_data)
        
        # SCM specific configuration
        self.pose_threshold = self.options_menu.pose_threshold
        hailo_logger.info(f"Using pose threshold: {self.pose_threshold}")
        
        # Resolve HEF path for pose detection model
        self.hef_path = resolve_hef_path(
            self.hef_path,
            app_name=POSE_ESTIMATION_PIPELINE,
            arch=self.arch
        )
        hailo_logger.info(f"Using HEF path: {self.hef_path}")
        
        # Note: Pose estimation typically doesn't need post-processing .so file
        # as keypoints are directly available from the model output
        self.post_process_so = None
        self.post_function_name = None
        
        self.app_callback = app_callback
        
        # Set the process title
        setproctitle.setproctitle("SCM Pose Detection v1")
        
        self.create_pipeline()

    def get_pipeline_string(self):
        """Construct the GStreamer pipeline for pose detection."""
        
        source_pipeline = SOURCE_PIPELINE(
            video_source=self.video_source,
            video_width=self.video_width,
            video_height=self.video_height,
            frame_rate=self.frame_rate,
            sync=self.sync,
            no_webcam_compression=True,
        )

        pose_pipeline = INFERENCE_PIPELINE(
            hef_path=self.hef_path,
            post_process_so=self.post_process_so,
            batch_size=self.batch_size,
            post_function_name=self.post_function_name,
            additional_params="",
            name="pose_detection",
        )

        user_callback_pipeline = USER_CALLBACK_PIPELINE()
        display_pipeline = DISPLAY_PIPELINE(video_sink=self.video_sink, sync=self.sync)

        pipeline_string = (
            f"{source_pipeline} ! "
            f"{pose_pipeline} ! "
            f"{user_callback_pipeline} ! "
            f"{display_pipeline}"
        )

        hailo_logger.info(f"Pipeline string: {pipeline_string}")
        return pipeline_string


# -----------------------------------------------------------------------------------------------
# Main execution and callback class
# -----------------------------------------------------------------------------------------------

class SCMUserCallbackClass(app_callback_class):
    """User callback class for SCM pose detection pipeline."""
    
    def __init__(self):
        super().__init__()
        self.pose_threshold = 0.5
        
    def set_pose_threshold(self, threshold):
        """Set the confidence threshold for pose keypoints."""
        self.pose_threshold = threshold


def scm_pose_callback(element, buffer, user_data):
    """
    Callback function for processing pose detection results.
    
    This function will be called for each frame with pose detection results.
    It extracts keypoints and derives body part bounding boxes.
    """
    hailo_logger.debug("Callback triggered. Current frame count=%d", user_data.get_count())
    try:
        # Get the GStreamer buffer
        if buffer is None:
            hailo_logger.warning("Received None buffer at frame=%s", user_data.get_count())
            return
            
        # Get frame information
        pad = element.get_static_pad("src")
        caps = get_caps_from_pad(pad)
        if caps is None:
            return Gst.PadProbeReturn.OK
        
        format, width, height = caps['format'], caps['width'], caps['height']
        hailo_logger.debug(f"Frame caps: format={format}, width={width}, height={height}")
            
        # Extract frame data (for debugging/visualization)
        frame = None
        if user_data.use_frame and format and width and height:
            frame = get_numpy_from_buffer(buffer, format, width, height)

        
        
        # Extract Hailo inference metadata from buffer
        # Note: The exact method to extract pose keypoints depends on the 
        # Hailo model output format. This is a placeholder implementation
        # that will need to be updated based on the actual model output.
        
        # For pose detection models, results are typically stored as:
        # - Detected person bounding boxes
        # - 17 keypoints per person (x, y, confidence)
        
        # Placeholder: simulate pose detection results for testing
        # In real implementation, this would come from buffer metadata
        simulated_poses = [
            {
                'person_bbox': (100, 100, 200, 400),  # x, y, w, h
                'keypoints': [
                    (150, 120, 0.9),  # nose
                    (140, 115, 0.8),  # left_eye
                    (160, 115, 0.8),  # right_eye
                    (135, 118, 0.7),  # left_ear
                    (165, 118, 0.7),  # right_ear
                    (120, 180, 0.9),  # left_shoulder
                    (180, 180, 0.9),  # right_shoulder
                    (110, 250, 0.8),  # left_elbow
                    (190, 250, 0.8),  # right_elbow
                    (100, 320, 0.7),  # left_wrist
                    (200, 320, 0.7),  # right_wrist
                    (130, 300, 0.9),  # left_hip
                    (170, 300, 0.9),  # right_hip
                    (125, 400, 0.8),  # left_knee
                    (175, 400, 0.8),  # right_knee
                    (120, 480, 0.7),  # left_ankle
                    (180, 480, 0.7),  # right_ankle
                ]
            }
        ]
        
        # Process detected poses
        for i, pose in enumerate(simulated_poses):
            keypoints = pose['keypoints']
            person_bbox = pose['person_bbox']
            
            # Filter keypoints by confidence threshold
            filtered_keypoints = [kp for kp in keypoints if kp[2] > user_data.pose_threshold]
            
            if len(filtered_keypoints) < 5:  # Need minimum keypoints for body parts
                hailo_logger.debug(f"Person {i}: Insufficient confident keypoints ({len(filtered_keypoints)})")
                continue
                
            # Derive body parts from keypoints
            body_parts = derive_body_parts_from_keypoints(keypoints)
            
            # Log results for verification
            hailo_logger.info(f"Person {i} detected:")
            hailo_logger.info(f"  - Confident keypoints: {len(filtered_keypoints)}/17")
            hailo_logger.info(f"  - Body parts detected: {list(body_parts.keys())}")
            
            for part_name, bbox in body_parts.items():
                x, y, w, h = bbox
                hailo_logger.info(f"  - {part_name}: bbox=({x:.0f}, {y:.0f}, {w:.0f}, {h:.0f})")
        
        return Gst.PadProbeReturn.OK
        
    except Exception as e:
        hailo_logger.error(f"Error in pose callback: {e}")
        return Gst.PadProbeReturn.OK


def main():
    """Main function to run the SCM pose detection pipeline."""
    
    # Create user data instance
    user_data = SCMUserCallbackClass()
    
    # Create and run the application
    app = SCMPoseDetectionApp(scm_pose_callback, user_data)
    
    # Set pose threshold from command line
    if hasattr(app.options_menu, 'pose_threshold'):
        user_data.set_pose_threshold(app.options_menu.pose_threshold)
    
    hailo_logger.info("Starting SCM Pose Detection Pipeline v1...")
    app.run()


if __name__ == "__main__":
    main()