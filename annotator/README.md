# Auto Annotator (Model Zoo)

Auto-labeling system using specialized single-class models to generate high-quality training data.

## Overview

The Model Zoo approach trains **one model per class** for better detection quality:

- Each model is a binary classifier (object vs background)
- No class imbalance issues per model
- Better bbox quality from specialization
- Used for auto-labeling, not deployment

## Directory Structure

```
annotator/
├── model/                  # Trained Model Zoo models
│   ├── hardhat_v1.pt
│   ├── gloves_v1.pt
│   ├── safetyvest_v1.pt
│   └── ...
├── input/                  # Images to auto-label
├── output/                 # Auto-labeled results
│   ├── images/             # Original images (copied)
│   ├── labels/             # YOLO format annotations
│   ├── preview/            # Annotated previews
│   └── data.yaml           # Class mapping
├── annotate.py             # Multi-model auto annotator
├── train.ipynb             # Model Zoo training notebook
└── README.md               # This file
```

## Usage

### 1. Training Models (train.ipynb)

Open `train.ipynb` in Jupyter and run the cells to train single-class models:

- Downloads binary datasets from Roboflow (one class per model)
- Trains with `single_cls=True` for binary classification
- Saves models as `{class_name}_v{version}.pt`

### 2. Auto-Labeling (annotate.py)

Run multiple models on images to generate annotations:

```bash
# Specific models
python annotate.py --models ./model/hardhat_v1.pt ./model/gloves_v1.pt --input ./input --output ./output

# All models via glob
python annotate.py --models ./model/*.pt --input ./input --output ./output

# With custom confidence threshold
python annotate.py --models ./model/*.pt --conf 0.4 --input ./input --output ./output
```

**Arguments:**

| Argument | Default | Description |
|----------|---------|-------------|
| `--models` | (required) | Paths to model files. Class name derived from filename. |
| `--input` | `./input` | Directory containing images to annotate |
| `--output` | `./output` | Output directory for results |
| `--conf` | `0.5` | Confidence threshold for detections |

**Class Naming:**

The class name is derived from the model filename:
- `hardhat.pt` → class `hardhat`
- `hardhat_v1.pt` → class `hardhat` (version suffix stripped)
- `safety_vest_v2.pt` → class `safety_vest`

### 3. Review & Upload

1. Check `output/preview/` for visual quality
2. Review and correct annotations in Roboflow
3. Export corrected dataset for deployment model training

## Output Format

**YOLO Annotation Format** (`output/labels/*.txt`):
```
<class_id> <x_center> <y_center> <width> <height>
```

All values are normalized (0-1). Example:
```
0 0.512500 0.341667 0.125000 0.183333
2 0.687500 0.558333 0.075000 0.116667
```

**data.yaml** (class mapping):
```yaml
nc: 3
names:
  0: hardhat
  1: gloves
  2: safetyvest
```

## Available Models

| Model | Class | Description |
|-------|-------|-------------|
| `person_v1.pt` | person | Person detection |
| `head_v1.pt` | head | Head/face area |
| `hand_v1.pt` | hand | Bare hands |
| `foot_v1.pt` | foot | Bare feet |
| `hardhat_v1.pt` | hardhat | Safety helmets |
| `gloves_v1.pt` | gloves | Safety gloves |
| `shoes_v1.pt` | shoes | Safety shoes |
| `safetyvest_v1.pt` | safetyvest | High-visibility vests |
| `facemask_v1.pt` | facemask | Face masks |

## Quality Assessment

After running `annotate.py`, check the detection rate:

| Rate | Assessment |
|------|------------|
| < 30% | Very low - model issues |
| 30-60% | Low - needs manual work |
| 60-80% | Acceptable for auto-labeling |
| > 80% | Good quality |

## Workflow

```
1. Collect images
   └── Place in input/

2. Run auto-labeler
   └── python annotate.py --models ./model/*.pt

3. Review output
   └── Check output/preview/

4. Upload to Roboflow
   └── Upload images/ and labels/

5. Human review
   └── Correct annotations in Roboflow

6. Export & train
   └── Train deployment model with corrected data
```
