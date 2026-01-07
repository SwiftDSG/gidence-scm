# region imports
# Standard library imports

# Third-party imports
import setproctitle
import numpy as np
import cv2
import gi

gi.require_version("Gst", "1.0")

# Local application-specific imports
import os
import hailo
from pathlib import Path
from gi.repository import Gst

from inference.core.common.core import (
    get_pipeline_parser, 
    get_resource_path
)
from inference.core.common.buffer_utils import get_caps_from_pad, get_numpy_from_buffer
from inference.core.common.defines import (
    POSE_ESTIMATION_PIPELINE,
    POSE_ESTIMATION_POSTPROCESS_FUNCTION,
    POSE_ESTIMATION_POSTPROCESS_SO_FILENAME,
    RESOURCES_SO_DIR_NAME,
)

from inference.core.common.hailo_logger import get_logger
from inference.core.gstreamer.gstreamer_app import (
    GStreamerApp,
    app_callback_class,
)
from inference.core.gstreamer.gstreamer_helper_pipelines import (
    DISPLAY_PIPELINE,
    INFERENCE_PIPELINE,
    INFERENCE_PIPELINE_WRAPPER,
    TRACKER_PIPELINE,
    SOURCE_PIPELINE,
    USER_CALLBACK_PIPELINE,
    
)

hailo_logger = get_logger(__name__)

# endregion imports

# -----------------------------------------------------------------------------------------------
# SCM Pose Detection Pipeline (Stage 1)
# -----------------------------------------------------------------------------------------------
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
            default=0.1,
            help="Confidence threshold for pose keypoints",
        )
        
        # Handle --list-models flag before full initialization
        # handle_list_models_flag(parser, POSE_ESTIMATION_PIPELINE)
        
        # Initialize parent
        super().__init__(parser, user_data)
        
        # SCM specific configuration
        self.pose_threshold = self.options_menu.pose_threshold
        hailo_logger.info(f"Using pose threshold: {self.pose_threshold}")

        # Model parameters - override defaults if not set via parser
        if self.batch_size == 1:
            self.batch_size = 2
        # video_width and video_height are already set from parser or defaults
        hailo_logger.info(
            "Video params set: %dx%d, batch_size=%d",
            self.video_width,
            self.video_height,
            self.batch_size,
        )
        
        # Use local HEF model from processor/model/ directory
        if self.hef_path is None:
            # Get the directory where this script is located
            script_dir = Path(__file__).parent
            # Go up to processor directory and into model subdirectory
            model_dir = script_dir.parent / "inference" / "model"
            self.hef_path = str(model_dir / "yolov8s_pose.hef")
        
        # Verify the model file exists
        if not os.path.exists(self.hef_path):
            hailo_logger.error(f"Model file not found: {self.hef_path}")
            raise FileNotFoundError(f"Model file not found: {self.hef_path}")
            
        hailo_logger.info(f"Using local HEF model: {self.hef_path}")
        
        # Note: Pose estimation typically doesn't need post-processing .so file
        # as keypoints are directly available from the model output

        # self.post_process_so = None
        # self.post_process_function = None
        self.post_process_so = get_resource_path(
            POSE_ESTIMATION_PIPELINE, RESOURCES_SO_DIR_NAME, self.arch, POSE_ESTIMATION_POSTPROCESS_SO_FILENAME
        )
        hailo_logger.info(f"Using post-process SO: {self.post_process_so}")
        self.post_process_function = POSE_ESTIMATION_POSTPROCESS_FUNCTION
        
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

        infer_pipeline = INFERENCE_PIPELINE(
            hef_path=self.hef_path,
            post_process_so=self.post_process_so,
            post_function_name=self.post_process_function,
            batch_size=self.batch_size,
            additional_params="",
            name="pose_detection",
        )

        infer_pipeline_wrapper = INFERENCE_PIPELINE_WRAPPER(infer_pipeline)
        tracker_pipeline = TRACKER_PIPELINE(class_id=0)
        user_callback_pipeline = USER_CALLBACK_PIPELINE()
        display_pipeline = DISPLAY_PIPELINE(
            video_sink=self.video_sink, sync=self.sync, show_fps=self.show_fps
        )

        pipeline_string = (
            f"{source_pipeline} ! "
            f"{infer_pipeline_wrapper} ! "
            f"{tracker_pipeline} ! "
            f"{user_callback_pipeline} ! "
            f"{display_pipeline}"
        )

        return pipeline_string


# -----------------------------------------------------------------------------------------------
# Main execution and callback class
# -----------------------------------------------------------------------------------------------
class SCMUserCallbackClass(app_callback_class):
    """User callback class for SCM pose detection pipeline."""
    
    def __init__(self):
        super().__init__()
        self.pose_threshold = 0.1
        
    def set_pose_threshold(self, threshold):
        """Set the confidence threshold for pose keypoints."""
        self.pose_threshold = threshold


# -----------------------------------------------------------------------------------------------
# User-defined callback function
# -----------------------------------------------------------------------------------------------
def scm_pose_callback(element, buffer, user_data):
    """
    Callback function for processing pose detection results.
    
    This function will be called for each frame with pose detection results.
    It extracts keypoints and derives body part bounding boxes.
    """
    hailo_logger.info("Callback triggered. Current frame count=%d", user_data.get_count())
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

        """Extract raw tensor outputs from Hailo"""
        # This is model-specific, example for YOLOv8:
        roi = hailo.get_roi_from_buffer(buffer)
        
        # Get tensor outputs (before post-processing)
        tensors = []
        for tensor in roi.get_tensors():
            hailo_logger.info("Tensor:", tensor)
            tensors.append({
                'data': tensor.data(),
                'shape': tensor.shape(),
                'name': tensor.name()
            })


        detections = roi.get_objects_typed(hailo.HAILO_DETECTION)
        hailo_logger.info("Number of detections: %d", len(detections))

        keypoints = get_keypoints()

        for detection in detections:
            label = detection.get_label()
            bbox = detection.get_bbox()
            confidence = detection.get_confidence()

            if label == "person":
                track_id = 0
                track = detection.get_objects_typed(hailo.HAILO_UNIQUE_ID)
                if len(track) == 1:
                    track_id = track[0].get_id()

                landmarks = detection.get_objects_typed(hailo.HAILO_LANDMARKS)
                if landmarks:
                    points = landmarks[0].get_points()
                    for eye in ["left_eye", "right_eye"]:
                        keypoint_index = keypoints[eye]
                        point = points[keypoint_index]
                        x = int((point.x() * bbox.width() + bbox.xmin()) * width)
                        y = int((point.y() * bbox.height() + bbox.ymin()) * height)
                        if user_data.use_frame:
                            cv2.circle(frame, (x, y), 5, (0, 255, 0), -1)

        if user_data.use_frame:
            frame = cv2.cvtColor(frame, cv2.COLOR_RGB2BGR)
            user_data.set_frame(frame)
        
        return Gst.PadProbeReturn.OK
        
    except Exception as e:
        hailo_logger.error(f"Error in pose callback: {e}")
        return Gst.PadProbeReturn.OK

def get_keypoints():
    return {
        "nose": 0,
        "left_eye": 1,
        "right_eye": 2,
        "left_ear": 3,
        "right_ear": 4,
        "left_shoulder": 5,
        "right_shoulder": 6,
        "left_elbow": 7,
        "right_elbow": 8,
        "left_wrist": 9,
        "right_wrist": 10,
        "left_hip": 11,
        "right_hip": 12,
        "left_knee": 13,
        "right_knee": 14,
        "left_ankle": 15,
        "right_ankle": 16,
    }

def main():
    """Main function to run the SCM pose detection pipeline."""
    
    # Create user data instance
    user_data = SCMUserCallbackClass()
    
    # Create and run the application
    app = SCMPoseDetectionApp(scm_pose_callback, user_data)
    
    hailo_logger.info("Starting SCM Pose Detection Pipeline v1...")
    app.run()


if __name__ == "__main__":
    main()