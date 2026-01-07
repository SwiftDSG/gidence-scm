#!/usr/bin/env python3
"""
Test script for the minimal hailo-apps detection_simple setup.
This script tests the import structure without requiring the hailo module.
"""

import sys
import traceback

def test_imports():
    """Test all the core imports without the hailo dependency"""
    
    print("Testing core module imports...")
    
    try:
        # Test core common modules
        print("  Testing hailo_apps.python.core.common...")
        from hailo_apps.python.core.common import defines
        from hailo_apps.python.core.common import hailo_logger
        from hailo_apps.python.core.common import parser
        from hailo_apps.python.core.common import core
        from hailo_apps.python.core.common import installation_utils
        from hailo_apps.python.core.common import buffer_utils
        from hailo_apps.python.core.common import camera_utils
        print("    ✓ Core common modules imported successfully")
        
        # Test gstreamer modules
        print("  Testing hailo_apps.python.core.gstreamer...")
        from hailo_apps.python.core.gstreamer import gstreamer_helper_pipelines
        from hailo_apps.python.core.gstreamer import gstreamer_common
        # Note: gstreamer_app requires gi which might not be available
        print("    ✓ GStreamer helper modules imported successfully")
        
        # Test config modules
        print("  Testing hailo_apps.config...")
        from hailo_apps.config import config_manager
        print("    ✓ Config modules imported successfully")
        
        print("\n✓ All core dependencies imported successfully!")
        print("\nNote: The actual detection_simple app requires:")
        print("  - hailo module (from HailoRT)")
        print("  - gi.repository.Gst (GStreamer Python bindings)")
        print("  - Proper GStreamer installation")
        
        return True
        
    except ImportError as e:
        print(f"\n✗ Import error: {e}")
        traceback.print_exc()
        return False
    except Exception as e:
        print(f"\n✗ Unexpected error: {e}")
        traceback.print_exc()
        return False

def test_logger():
    """Test the logging system"""
    print("\nTesting logging system...")
    try:
        from hailo_apps.python.core.common.hailo_logger import get_logger
        logger = get_logger("test_minimal")
        logger.info("Test log message")
        print("✓ Logging system working")
        return True
    except Exception as e:
        print(f"✗ Logging test failed: {e}")
        return False

def test_config():
    """Test configuration loading"""
    print("\nTesting configuration system...")
    try:
        from hailo_apps.config.config_manager import get_default_models
        # This might fail if config files are not properly set up
        print("✓ Config manager imported")
        return True
    except Exception as e:
        print(f"✗ Config test failed: {e}")
        return False

def main():
    """Main test function"""
    print("Hailo Apps Minimal - Test Suite")
    print("=" * 40)
    
    all_passed = True
    
    # Test core imports
    all_passed &= test_imports()
    
    # Test logger
    all_passed &= test_logger()
    
    # Test config (may fail without proper setup)
    test_config()  # Don't fail on this
    
    print("\n" + "=" * 40)
    if all_passed:
        print("✓ Core tests passed! The minimal setup structure is correct.")
        print("\nTo complete the setup:")
        print("1. Install HailoRT")
        print("2. Run ./install.sh")
        print("3. Use: source ./setup_env.sh && hailo-detect-simple")
    else:
        print("✗ Some tests failed. Check the errors above.")
        sys.exit(1)

if __name__ == "__main__":
    main()