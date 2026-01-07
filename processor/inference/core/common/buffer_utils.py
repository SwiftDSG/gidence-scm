"""
Buffer utilities for GStreamer processing.
Contains functions for extracting data from GStreamer buffers and pads.
"""

import numpy as np
import gi
gi.require_version("Gst", "1.0")
from gi.repository import Gst

from .defines import (
    HAILO_NV12_VIDEO_FORMAT,
    HAILO_RGB_VIDEO_FORMAT, 
    HAILO_YUYV_VIDEO_FORMAT,
)
from .hailo_logger import get_logger

logger = get_logger(__name__)


def get_caps_from_pad(pad):
    """
    Get caps from a GStreamer pad.
    
    Args:
        pad: GStreamer pad object
        
    Returns:
        Gst.Caps: The caps from the pad
    """
    caps = pad.get_current_caps()
    if not caps:
        caps = pad.query_caps(None)
    return caps


def get_numpy_from_buffer(buffer, caps):
    """
    Extract numpy array from GStreamer buffer.
    
    Args:
        buffer: GStreamer buffer object
        caps: GStreamer caps object
        
    Returns:
        numpy.ndarray: The extracted numpy array
    """
    # Extract buffer info
    success, buffer_map = buffer.map(Gst.MapFlags.READ)
    if not success:
        logger.error("Failed to map buffer")
        return None
        
    try:
        # Get video info from caps
        video_info = get_video_info_from_caps(caps)
        if not video_info:
            logger.error("Failed to get video info from caps")
            return None
            
        # Extract numpy array based on format
        numpy_array = extract_numpy_array(buffer_map, video_info)
        return numpy_array
        
    finally:
        buffer.unmap(buffer_map)


def get_video_info_from_caps(caps):
    """
    Extract video info from GStreamer caps.
    
    Args:
        caps: GStreamer caps object
        
    Returns:
        dict: Video information (width, height, format)
    """
    if not caps or caps.get_size() == 0:
        return None
        
    structure = caps.get_structure(0)
    if not structure:
        return None
        
    width = structure.get_int("width")[1] if structure.get_int("width")[0] else None
    height = structure.get_int("height")[1] if structure.get_int("height")[0] else None
    format_str = structure.get_string("format")
    
    if not all([width, height, format_str]):
        return None
        
    return {
        "width": width,
        "height": height, 
        "format": format_str
    }


def extract_numpy_array(buffer_map, video_info):
    """
    Extract numpy array from mapped buffer data.
    
    Args:
        buffer_map: Mapped GStreamer buffer
        video_info: Video information dict
        
    Returns:
        numpy.ndarray: The extracted array
    """
    width = video_info["width"]
    height = video_info["height"]
    format_str = video_info["format"]
    
    # Get raw buffer data
    buffer_data = buffer_map.data
    
    if format_str in [HAILO_RGB_VIDEO_FORMAT]:
        # RGB format
        array = np.frombuffer(buffer_data, dtype=np.uint8)
        array = array.reshape((height, width, 3))
        
    elif format_str in [HAILO_NV12_VIDEO_FORMAT]:
        # NV12 format (Y plane + interleaved UV)
        array = np.frombuffer(buffer_data, dtype=np.uint8)
        # For NV12, we typically just return the Y plane
        y_plane_size = width * height
        array = array[:y_plane_size].reshape((height, width))
        
    elif format_str in [HAILO_YUYV_VIDEO_FORMAT]:
        # YUYV format
        array = np.frombuffer(buffer_data, dtype=np.uint8)
        # YUYV has 2 bytes per pixel
        array = array.reshape((height, width, 2))
        
    else:
        logger.warning(f"Unsupported video format: {format_str}")
        # Fallback: return raw buffer as 1D array
        array = np.frombuffer(buffer_data, dtype=np.uint8)
        
    return array