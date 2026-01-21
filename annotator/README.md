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
│   ├── hardhat.pt
│   ├── gloves.pt
│   ├── safetyvest.pt
│   └── ...
├── input/                  # Images to auto-label (Generate manually)
├── output/                 # Auto-labeled results (Automatically generated)
│   ├── images/             # Original images (copied)
│   ├── labels/             # YOLO format annotations
│   ├── preview/            # Annotated previews
│   └── data.yaml           # Class mapping
├── config.json             # Configuration file
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

### 2. Configure (config.json)

Edit `config.json` to set up your auto-labeling run:

```json
{
  "input": "./input",
  "output": "./output",
  "defaults": {
    "conf": 0.5
  },
  "models": {
    "hardhat": {
      "path": "./model/hardhat.pt",
      "conf": 0.5,
      "size_rules": [
        [0.02, 0.25],
        [0.05, 0.35]
      ]
    },
    "gloves": {
      "path": "./model/gloves.pt",
      "conf": 0.4
    }
  }
}
```

**Configuration Options:**

| Field      | Description                               |
| ---------- | ----------------------------------------- |
| `input`    | Directory containing images to annotate   |
| `output`   | Output directory for results              |
| `defaults` | Default settings (e.g., `conf` threshold) |
| `models`   | Per-class model config (see below)        |

**Model Configuration:**

| Field        | Description                              |
| ------------ | ---------------------------------------- |
| `path`       | Path to the model file (.pt)             |
| `conf`       | Confidence threshold (optional)          |
| `size_rules` | Size-based confidence rules (optional)   |

### 3. Size-Based Confidence Rules

For overhead cameras where objects appear smaller at distance, you can set lower confidence thresholds for small detections:

```json
"hardhat": {
  "conf": 0.5,
  "size_rules": [
    [0.02, 0.25],
    [0.05, 0.35]
  ]
}
```

Each rule is `[max_width_ratio, confidence]`.

**How it works:**

| Detection Width      | Confidence Required |
| -------------------- | ------------------- |
| < 2% of image width  | 0.25 (very small)   |
| 2-5% of image width  | 0.35 (small)        |
| > 5% of image width  | 0.50 (base)         |

This helps catch small/distant objects that would otherwise be missed due to model uncertainty at small scales.

### 4. Auto-Labeling (annotate.py)

Run the auto-annotator:

```bash
# Use default config (./config.json)
python annotate.py

# Use custom config path
python annotate.py --config ./my_config.json
```

### 5. Review & Upload

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

| Model           | Class      | Description           |
| --------------- | ---------- | --------------------- |
| `person.pt`     | person     | Person detection      |
| `head.pt`       | head       | Head/face area        |
| `hand.pt`       | hand       | Bare hands            |
| `foot.pt`       | foot       | Bare feet             |
| `hardhat.pt`    | hardhat    | Safety helmets        |
| `gloves.pt`     | gloves     | Safety gloves         |
| `shoes.pt`      | shoes      | Safety shoes          |
| `safetyvest.pt` | safetyvest | High-visibility vests |
| `facemask.pt`   | facemask   | Face masks            |

## Quality Assessment

After running `annotate.py`, check the detection rate:

| Rate   | Assessment                   |
| ------ | ---------------------------- |
| < 30%  | Very low - model issues      |
| 30-60% | Low - needs manual work      |
| 60-80% | Acceptable for auto-labeling |
| > 80%  | Good quality                 |

## Workflow

```
1. Collect images
   └── Place in input/

2. Configure
   └── Edit config.json (models, paths, confidence)

3. Run auto-labeler
   └── python annotate.py

4. Review output
   └── Check output/preview/

5. Upload to Roboflow
   └── Upload images/ and labels/

6. Human review
   └── Correct annotations in Roboflow

7. Export & train
   └── Train deployment model with corrected data
```
