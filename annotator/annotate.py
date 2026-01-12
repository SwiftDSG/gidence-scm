#!/usr/bin/env python3
"""
Test Model and Export Roboflow-Ready Annotations

This script:
1. Runs your trained model on images
2. Generates YOLO-format annotations (.txt files)
3. Saves annotated images for visual inspection
4. Creates a folder ready to upload to Roboflow

Usage:
    python test_and_export_annotations.py --model best.pt --images ./test_images --output ./roboflow_ready
"""

import argparse
from pathlib import Path
from ultralytics import YOLO
import shutil
import cv2
import numpy as np

def export_roboflow_annotations(model_path, image_dir, output_dir, conf_threshold=0.25):
    """
    Run model on images and export Roboflow-compatible annotations.
    
    Args:
        model_path: Path to trained model (.pt file)
        image_dir: Directory containing images to annotate
        output_dir: Where to save Roboflow-ready dataset
        conf_threshold: Confidence threshold for detections
    """
    
    # Load model
    print(f"Loading model: {model_path}")
    model = YOLO(model_path)
    
    # Get class names from model
    class_names = model.names
    print(f"Model classes: {class_names}")
    
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
    
    print(f"\nFound {len(image_files)} images to process")
    
    if len(image_files) == 0:
        print(f"ERROR: No images found in {image_dir}")
        return
    
    # Process each image
    stats = {
        'total_images': len(image_files),
        'images_with_detections': 0,
        'total_detections': 0,
        'detections_per_class': {name: 0 for name in class_names.values()}
    }
    
    for i, img_path in enumerate(image_files, 1):
        print(f"Processing {i}/{len(image_files)}: {img_path.name}", end='... ')
        
        # Run inference
        results = model.predict(
            source=str(img_path),
            conf=conf_threshold,
            iou=0.4, # NMS IOU threshold (lower = fewer duplicates)
            verbose=False
        )
        
        result = results[0]
        
        # Check if any detections
        if result.boxes is None or len(result.boxes) == 0:
            print("No detections")
            continue
        
        num_detections = len(result.boxes)
        stats['images_with_detections'] += 1
        stats['total_detections'] += num_detections
        
        print(f"{num_detections} detections")
        
        # Copy original image
        dst_image = images_dir / img_path.name
        shutil.copy(img_path, dst_image)
        
        # Create YOLO format annotations
        img_height, img_width = result.orig_shape
        annotations = []
        
        for box in result.boxes:
            # Get class ID and confidence
            class_id = int(box.cls[0])
            confidence = float(box.conf[0])
            
            # Get bbox in xyxy format
            x1, y1, x2, y2 = box.xyxy[0].tolist()
            
            # Convert to YOLO format (normalized xywh)
            x_center = ((x1 + x2) / 2) / img_width
            y_center = ((y1 + y2) / 2) / img_height
            width = (x2 - x1) / img_width
            height = (y2 - y1) / img_height
            
            # Create annotation line
            annotation = f"{class_id} {x_center:.6f} {y_center:.6f} {width:.6f} {height:.6f}"
            annotations.append(annotation)
            
            # Update stats
            class_name = class_names[class_id]
            stats['detections_per_class'][class_name] += 1
        
        # Save annotations to .txt file
        label_file = labels_dir / f"{img_path.stem}.txt"
        with open(label_file, 'w') as f:
            f.write('\n'.join(annotations))
        
        # Save annotated image for visual inspection
        annotated_img = result.plot()
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
        if count > 0:
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
    print(f"4. Check annotated_preview/ to verify quality before uploading")
    print("="*70)
    
    # Create a simple data.yaml for reference
    data_yaml = output_dir / 'data.yaml'
    with open(data_yaml, 'w') as f:
        f.write("# Auto-generated from model predictions\n")
        f.write(f"# Model: {model_path}\n")
        f.write(f"# Confidence threshold: {conf_threshold}\n\n")
        f.write("train: images\n")
        f.write("val: images\n\n")
        f.write(f"nc: {len(class_names)}\n")
        f.write("names:\n")
        for class_id, class_name in class_names.items():
            f.write(f"  {class_id}: {class_name}\n")
    
    print(f"\ndata.yaml created: {data_yaml}")
    
    return stats


def main():
    parser = argparse.ArgumentParser(description='Export Roboflow-ready annotations from model')
    parser.add_argument('--model', default='./annotator/best.pt', help='Path to trained model (.pt file)')
    parser.add_argument('--images', default='./annotator/input', help='Directory containing images')
    parser.add_argument('--output', default='./annotator/output', help='Output directory')
    parser.add_argument('--conf', type=float, default=0.5, help='Confidence threshold')
    
    args = parser.parse_args()
    
    # Run export
    stats = export_roboflow_annotations(
        model_path=args.model,
        image_dir=args.images,
        output_dir=args.output,
        conf_threshold=args.conf
    )
    
    # Evaluation
    if stats:
        print("\n" + "="*70)
        print("QUALITY ASSESSMENT")
        print("="*70)
        
        detection_rate = stats['images_with_detections'] / stats['total_images'] * 100
        
        print(f"Detection rate: {detection_rate:.1f}%")
        
        if detection_rate < 30:
            print("❌ VERY LOW - Most images have no detections!")
            print("   → Model might not be working well")
            print("   → Check confidence threshold or model quality")

        elif detection_rate < 60:
            print("⚠️  LOW - Many images have no detections")
            print("   → Model is struggling, but might be usable for auto-labeling")
            print("   → Human will need to add many missing annotations")

        elif detection_rate < 80:
            print("✓ ACCEPTABLE - Most images have detections")
            print("   → Good enough for auto-labeling")
            print("   → Human will verify and add missed objects")

        else:
            print("✓✓ GOOD - Almost all images have detections")
            print("   → Should work well for auto-labeling")
        
        print("\n⚠️  IMPORTANT:")
        print("   1. Check ./annotator/output/preview folder for visual quality")
        print("   2. Make sure detections look reasonable")
        print("   3. Upload to Roboflow for human review/correction")
        print("="*70)


if __name__ == "__main__":
    main()