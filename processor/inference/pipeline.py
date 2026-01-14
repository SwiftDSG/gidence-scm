# region imports
# Standard library imports
import setproctitle
import os
from pathlib import Path

from inference.core.common.core import get_pipeline_parser, get_resource_path, handle_list_models_flag, resolve_hef_path
from inference.core.common.defines import (
    TAPPAS_POSTPROC_PATH_KEY,
    TAPPAS_STREAM_ID_TOOL_SO_FILENAME
)

from inference.core.common.hailo_logger import get_logger
from inference.core.gstreamer.gstreamer_app import (
    GStreamerApp,
    app_callback_class,
    dummy_callback,
)
from inference.core.gstreamer.gstreamer_helper_pipelines import (
    DISPLAY_PIPELINE,
    INFERENCE_PIPELINE,
    SOURCE_PIPELINE,
    TRACKER_PIPELINE,
    USER_CALLBACK_PIPELINE,
    QUEUE
)

hailo_logger = get_logger(__name__)

# endregion imports

# -----------------------------------------------------------------------------------------------
# User Gstreamer Application
# -----------------------------------------------------------------------------------------------


# This class inherits from the hailo_rpi_common.GStreamerApp class
class SCMPipeline(GStreamerApp):
    def __init__(self, app_callback, user_data, cameras, model="yolov8n.hef", parser=None):
        if parser is None:
            parser = get_pipeline_parser()
        parser.add_argument(
            "--labels-json",
            default=None,
            help="Path to costume labels JSON file",
        )
        
        hailo_logger.info("Initializing GStreamer Detection Simple App...")
        super().__init__(parser, user_data)
        # Override width/height if not set via parser
        if self.video_width == 1280:
            self.video_width = 640
        if self.video_height == 720:
            self.video_height = 640

        # Set Hailo parameters - these parameters should be set based on the model used
        # Override batch_size if not set via parser
        if self.batch_size == 1:
            self.batch_size = 2
        nms_score_threshold = 0.3
        nms_iou_threshold = 0.45

        self.thresholds_str = (
            f"nms-score-threshold={nms_score_threshold} "
            f"nms-iou-threshold={nms_iou_threshold} "
            f"output-format-type=HAILO_FORMAT_TYPE_FLOAT32"
        )

        hailo_logger.info(f"Using thresholds: {self.thresholds_str}")

        script_dir = Path(__file__).parent

        self.cameras = cameras
        self.video_sources = []
        if len(self.cameras) == 0:  # No cameras specified, use default video
            hailo_logger.warning("No cameras specified, using default video source.")
            self.video_sources.append(str(script_dir.parent / "input.mp4"))
        else:
            for cam in self.cameras:
                hailo_logger.info(f"Configured camera: {cam['id']} - {cam['name']} @ {cam['url']}")
                self.video_sources.append(cam["url"])

        # Use local HEF model from processor/inference/model/ directory
        model_dir = script_dir.parent / "inference" / "model"
        self.hef_path = str(model_dir / model)
        
        # Verify the model file exists
        if not os.path.exists(self.hef_path):
            hailo_logger.error(f"Model file not found: {self.hef_path}")
            raise FileNotFoundError(f"Model file not found: {self.hef_path}")
            
        hailo_logger.info(f"Using local HEF model: {self.hef_path}")
        
        # Use local post-processing SO from processor/inference/so/ directory
        so_dir = script_dir.parent / "inference" / "so"
        self.post_process_so = str(so_dir / "yolov8.so")
        self.post_process_function = "filter"

        # Verify the so file exists
        if not os.path.exists(self.post_process_so):
            hailo_logger.error(f"Post-process SO file not found: {self.post_process_so}")
            raise FileNotFoundError(f"Post-process SO file not found: {self.post_process_so}")

        # User-defined label JSON file
        self.labels_json = self.options_menu.labels_json
        hailo_logger.info(f"Using labels JSON file: {self.labels_json}")

        self.app_callback = app_callback

        # Set the process title
        setproctitle.setproctitle("SCM")

        self.create_pipeline()

    def get_pipeline_string(self):
        sources_string = ""
        router_string = ""

        tappas_post_process_dir = os.environ.get(TAPPAS_POSTPROC_PATH_KEY, '')
        set_stream_id_so = os.path.join(tappas_post_process_dir, TAPPAS_STREAM_ID_TOOL_SO_FILENAME)
        for i, src in enumerate(self.video_sources):
            sources_string += SOURCE_PIPELINE(
                video_source=src,
                video_width=self.video_width,
                video_height=self.video_height,
                frame_rate=self.frame_rate,
                sync=self.sync,
                no_webcam_compression=True,
            )
            id = self.cameras[i]["id"] if i < len(self.cameras) else f"input"
            sources_string += f"! hailofilter name=set_src_{id} so-path={set_stream_id_so} config-path=src_{id} "
            sources_string += f"! {QUEUE(name=f'src_q_{id}', max_size_buffers=30)} ! robin.sink_{id} "
            router_string += f"router.src_{id} ! {USER_CALLBACK_PIPELINE(name=f'src_{id}_callback')} ! {QUEUE(name=f'callback_q_{id}', max_size_buffers=30)} ! {DISPLAY_PIPELINE(video_sink=self.video_sink, sync=self.sync, show_fps=self.show_fps, name=f'hailo_display_{id}')} "

        detection_pipeline = INFERENCE_PIPELINE(
            hef_path=self.hef_path,
            post_process_so=self.post_process_so,
            post_function_name=self.post_process_function,
            batch_size=self.batch_size,
            config_json=self.labels_json,
            additional_params=self.thresholds_str,
        )
        tracker_pipeline = TRACKER_PIPELINE(class_id = 0)
        user_callback_pipeline = USER_CALLBACK_PIPELINE()
        # display_pipeline = DISPLAY_PIPELINE(video_sink=self.video_sink, sync=self.sync, show_fps=self.show_fps)

        inference_string = f"hailoroundrobin mode=1 name=robin ! {detection_pipeline} ! {tracker_pipeline} ! {user_callback_pipeline} ! {QUEUE(name='call_q', max_size_buffers=30)} ! hailostreamrouter name=router "

        for i, _ in enumerate(self.video_sources):
            inference_string += f"src_{i}::input-streams=\"<sink_{i}>\" "

        pipeline_string = sources_string + inference_string + router_string

        hailo_logger.info(f"Pipeline string: {pipeline_string}")
        return pipeline_string