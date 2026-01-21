#!/usr/bin/env python3
"""
Multi-Model Auto Annotator

This script:
1. Runs multiple single-class models on images
2. Uses the model filename as the class name (e.g., hardhat.pt → "hardhat")
3. Generates YOLO-format annotations (.txt files)
4. Saves annotated images for visual inspection
5. Creates a folder ready to upload to Roboflow

Usage:
    python annotate.py
    python annotate.py --config ./config.json
"""

import argparse
import json
from pathlib import Path
from ultralytics import YOLO
import shutil
import cv2


def get_confidence_for_detection(class_name, box_width, img_width, config):
    """
    Get the appropriate confidence threshold for a detection based on its size.

    Args:
        class_name: Name of the detected class
        box_width: Width of the bounding box in pixels
        img_width: Width of the image in pixels
        config: Configuration dictionary

    Returns:
        Confidence threshold to apply for this detection
    """
    defaults = config.get('defaults', {})
    default_conf = defaults.get('conf', 0.5)

    model_config = config.get('models', {}).get(class_name, {})
    base_conf = model_config.get('conf', default_conf)

    size_rules = model_config.get('size_rules', [])
    if not size_rules:
        return base_conf

    # Calculate width ratio
    width_ratio = box_width / img_width

    # Find applicable rule (sorted by max_width_ratio ascending)
    # Format: [max_width_ratio, confidence]
    for max_ratio, conf in sorted(size_rules, key=lambda x: x[0]):
        if width_ratio < max_ratio:
            return conf

    # No size rule matched, use base confidence
    return base_conf


def get_min_confidence_for_model(class_name, config):
    """
    Get the minimum confidence threshold for a model (for initial inference).
    This ensures we don't filter out small objects prematurely.

    Args:
        class_name: Name of the class
        config: Configuration dictionary

    Returns:
        Minimum confidence threshold to use for inference
    """
    defaults = config.get('defaults', {})
    default_conf = defaults.get('conf', 0.5)

    model_config = config.get('models', {}).get(class_name, {})
    base_conf = model_config.get('conf', default_conf)

    size_rules = model_config.get('size_rules', [])
    if not size_rules:
        return base_conf

    # Return the minimum of base_conf and all size rule confidences
    # Format: [max_width_ratio, confidence]
    all_confs = [base_conf] + [rule[1] for rule in size_rules]
    return min(all_confs)


def export_roboflow_annotations(config):
    """
    Run multiple models on images and export Roboflow-compatible annotations.

    Args:
        config: Configuration dictionary with all settings
    """
    models_config = config.get('models', {})
    image_dir = config.get('input', './input')
    output_dir = config.get('output', './output')

    if not models_config:
        print("ERROR: No models specified in config")
        return None

    # Load all models
    print("Loading models...")
    models = {}  # {class_name: model}
    class_names = {}  # {class_id: class_name}

    for i, (class_name, model_cfg) in enumerate(models_config.items()):
        model_path = Path(model_cfg.get('path', ''))

        if not model_path.exists():
            print(f"  WARNING: Model not found: {model_path}")
            continue

        print(f"  [{i}] {class_name} → {model_path}")

        model = YOLO(str(model_path))
        models[class_name] = model
        class_names[i] = class_name

    if not models:
        print("ERROR: No valid models loaded")
        return None

    print(f"\nLoaded {len(models)} models with classes: {list(models.keys())}")

    # Create reverse mapping for class_id lookup
    class_to_id = {name: i for i, name in class_names.items()}

    # Setup output directories
    output_dir = Path(output_dir)
    images_dir = output_dir / 'images'
    labels_dir = output_dir / 'labels'
    annotated_dir = output_dir / 'preview'

    images_dir.mkdir(parents=True, exist_ok=True)
    labels_dir.mkdir(parents=True, exist_ok=True)
    annotated_dir.mkdir(parents=True, exist_ok=True)

    # Get all images
    image_dir = Path(image_dir)
    image_files = []
    for ext in ['*.jpg', '*.jpeg', '*.png', '*.JPG', '*.JPEG', '*.PNG']:
        image_files.extend(list(image_dir.glob(ext)))

    image_count = len(image_files)
    print(f"\nFound {image_count} images to process")

    if image_count == 0:
        print(f"ERROR: No images found in {image_dir}")
        return None

    # Process each image
    stats = {
        'total_images': image_count,
        'images_with_detections': 0,
        'total_detections': 0,
        'detections_per_class': {name: 0 for name in class_names.values()}
    }

    for i, img_path in enumerate(image_files, 1):
        print(f"Processing {i}/{image_count}: {img_path.name}", end='... ')

        # Load image once
        img = cv2.imread(str(img_path))
        if img is None:
            print("Failed to load image")
            continue

        img_height, img_width = img.shape[:2]

        # Run all models and collect detections
        all_annotations = []
        all_boxes = []  # For visualization: (x1, y1, x2, y2, class_name, conf)

        for class_name, model in models.items():
            class_id = class_to_id[class_name]

            # Get minimum confidence for this model (to catch small objects)
            min_conf = get_min_confidence_for_model(class_name, config)

            # Run inference with minimum confidence
            results = model.predict(
                source=img,
                conf=min_conf,
                iou=0.4,  # NMS IOU threshold
                verbose=False
            )

            result = results[0]

            # Skip if no detections for this model
            if result.boxes is None or len(result.boxes) == 0:
                continue

            for box in result.boxes:
                confidence = float(box.conf[0])

                # Get bbox in xyxy format
                x1, y1, x2, y2 = box.xyxy[0].tolist()
                box_width = x2 - x1

                # Get the appropriate confidence threshold for this detection's size
                required_conf = get_confidence_for_detection(class_name, box_width, img_width, config)

                # Skip if confidence doesn't meet the size-based threshold
                if confidence < required_conf:
                    continue

                # Convert to YOLO format (normalized xywh)
                x_center = ((x1 + x2) / 2) / img_width
                y_center = ((y1 + y2) / 2) / img_height
                width = (x2 - x1) / img_width
                height = (y2 - y1) / img_height

                # Create annotation line with the class_id from filename
                annotation = f"{class_id} {x_center:.6f} {y_center:.6f} {width:.6f} {height:.6f}"
                all_annotations.append(annotation)

                # Store for visualization
                all_boxes.append((int(x1), int(y1), int(x2), int(y2), class_name, confidence))

                # Update stats
                stats['detections_per_class'][class_name] += 1

        num_detections = len(all_annotations)

        if num_detections == 0:
            print("No detections")
            continue

        stats['images_with_detections'] += 1
        stats['total_detections'] += num_detections
        print(f"{num_detections} detections")

        # Copy original image
        dst_image = images_dir / img_path.name
        shutil.copy(img_path, dst_image)

        # Save annotations to .txt file
        label_file = labels_dir / f"{img_path.stem}.txt"
        with open(label_file, 'w') as f:
            f.write('\n'.join(all_annotations))

        # Draw annotations for preview
        annotated_img = img.copy()
        colors = {}  # Cache colors per class

        for (x1, y1, x2, y2, class_name, conf) in all_boxes:
            # Generate consistent color per class
            if class_name not in colors:
                hash_val = hash(class_name)
                colors[class_name] = (
                    (hash_val & 0xFF),
                    ((hash_val >> 8) & 0xFF),
                    ((hash_val >> 16) & 0xFF)
                )
            color = colors[class_name]

            # Draw box
            cv2.rectangle(annotated_img, (x1, y1), (x2, y2), color, 2)

            # Draw label
            label = f"{class_name} {conf:.2f}"
            (label_w, label_h), baseline = cv2.getTextSize(label, cv2.FONT_HERSHEY_SIMPLEX, 0.5, 1)
            cv2.rectangle(annotated_img, (x1, y2), (x1 + label_w, y2 + label_h + 5), color, -1)
            cv2.putText(annotated_img, label, (x1, y2 + label_h + 5), cv2.FONT_HERSHEY_SIMPLEX, 0.5, (255, 255, 255), 1)

        # Save annotated preview
        annotated_path = annotated_dir / img_path.name
        cv2.imwrite(str(annotated_path), annotated_img)

    # Print summary
    print("\n" + "="*70)
    print("ANNOTATION EXPORT COMPLETE")
    print("="*70)
    print(f"Total images processed: {stats['total_images']}")
    print(f"Images with detections: {stats['images_with_detections']}")
    print(f"Images without detections: {stats['total_images'] - stats['images_with_detections']}")
    print(f"Total detections: {stats['total_detections']}")

    if stats['images_with_detections'] > 0:
        avg_det = stats['total_detections'] / stats['images_with_detections']
        print(f"Average detections per image: {avg_det:.1f}")

    print("\nDetections per class:")
    for class_name, count in sorted(stats['detections_per_class'].items(), key=lambda x: x[1], reverse=True):
        print(f"  {class_name:20s}: {count:5d}")

    print("\n" + "="*70)
    print("OUTPUT FILES:")
    print("="*70)
    print(f"Images: {images_dir}")
    print(f"Labels: {labels_dir}")
    print(f"Annotated preview: {annotated_dir}")

    print("\n" + "="*70)
    print("READY FOR ROBOFLOW UPLOAD")
    print("="*70)
    print(f"1. Upload images from: {images_dir}")
    print(f"2. Upload labels from: {labels_dir}")
    print(f"3. Or zip both folders and upload to Roboflow")
    print(f"4. Check preview/ to verify quality before uploading")
    print("="*70)

    # Create data.yaml for reference
    data_yaml = output_dir / 'data.yaml'
    with open(data_yaml, 'w') as f:
        f.write("# Auto-generated from multi-model predictions\n")
        f.write("# See config.json for model paths and confidence settings\n\n")
        f.write("train: images\n")
        f.write("val: images\n\n")
        f.write(f"nc: {len(class_names)}\n")
        f.write("names:\n")
        for class_id, class_name in class_names.items():
            f.write(f"  {class_id}: {class_name}\n")

    print(f"\ndata.yaml created: {data_yaml}")

    return stats


def load_config(config_path):
    """Load configuration from JSON file."""
    config_path = Path(config_path)
    if not config_path.exists():
        print(f"ERROR: Config file not found at {config_path}")
        return None

    with open(config_path, 'r') as f:
        config = json.load(f)

    print(f"Loaded config from {config_path}")
    print(f"  Input:  {config.get('input', './input')}")
    print(f"  Output: {config.get('output', './output')}")
    print(f"  Models: {len(config.get('models', []))} configured")
    print()

    return config


def main():
    parser = argparse.ArgumentParser(description='Multi-model auto annotator for Roboflow')
    parser.add_argument('--config', default='./config.json',
                        help='Path to config JSON file (default: ./config.json)')

    args = parser.parse_args()

    # Load configuration
    config = load_config(args.config)
    if config is None:
        return

    # Run export
    stats = export_roboflow_annotations(config)

    # Evaluation
    if stats:
        print("\n" + "="*70)
        print("QUALITY ASSESSMENT")
        print("="*70)

        detection_rate = stats['images_with_detections'] / stats['total_images'] * 100

        print(f"Detection rate: {detection_rate:.1f}%")

        if detection_rate < 30:
            print("Very low - Most images have no detections!")
            print("   -> Model might not be working well")
            print("   -> Check confidence threshold or model quality")

        elif detection_rate < 60:
            print("Low - Many images have no detections")
            print("   -> Model is struggling, but might be usable for auto-labeling")
            print("   -> Human will need to add many missing annotations")

        elif detection_rate < 80:
            print("Acceptable - Most images have detections")
            print("   -> Good enough for auto-labeling")
            print("   -> Human will verify and add missed objects")

        else:
            print("Good - Almost all images have detections")
            print("   -> Should work well for auto-labeling")

        print("\nIMPORTANT:")
        print("   1. Check ./output/preview folder for visual quality")
        print("   2. Make sure detections look reasonable")
        print("   3. Upload to Roboflow for human review/correction")
        print("="*70)


if __name__ == "__main__":
    main()
