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

from inference.core.common.buffer_utils import get_numpy_from_buffer
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

        for detection in detections:
            string_to_print += (
                f"Detection: {detection.get_label()} Confidence: {detection.get_confidence():.2f}\n"
            )
        
        print(string_to_print)
        return
            
    except Exception as e:
        hailo_logger.error(f"Error in pose callback: {e}")
        return Gst.PadProbeReturn.OK


def main():
    hailo_logger.info("Starting Detection Simple App.")
    user_data = SCMCallback()
    app = GStreamerDetectionSimpleApp(scm_callback, user_data)
    app.run()

if __name__ == "__main__":
    main()
