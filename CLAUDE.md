# SCM Pipeline Development with Hailo - Model Zoo Approach

**Project:** Safety Compliance Monitoring (SCM) - Detection-Based PPE System  
**Hardware:** Raspberry Pi 5 + Hailo-8 AI Accelerator (26 TOPS)  
**Approach:** Model Zoo for training, Single model for deployment  
**Timeline:** January 8-24, 2026 (16 days remaining)

---

## MAJOR DIRECTION CHANGE (Jan 12, 2026)

### **What Changed**

**Original Plan (Abandoned):**

- ❌ Three-stage pipeline (Pose → PPE → Association)
- ❌ YOLOv8-pose for keypoint detection
- ❌ Complex keypoint-based body part derivation
- ❌ Three separate models (pose + common PPE + rare PPE)

**New Plan (Current):**

- ✅ Detection-only approach (no pose estimation)
- ✅ Model Zoo for training (specialized single-class models)
- ✅ Single 17-class model for deployment
- ✅ Body part detection (head, hand, foot, face, ear) AS classes
- ✅ Visibility-aware compliance checking

### **Why We Changed**

**Problem Discovered:**

- Overhead cameras (8-10m height, 45-60° angle) at Unilever
- YOLOv8-pose trained on ground-level views (COCO dataset)
- Small person size (30-80 pixels) + unusual angles
- Pose estimation fails completely in real deployment!

**Solution:**

- Use detection-only (works better at small scales)
- SH17 dataset already includes body parts as classes!
- Simpler, faster, more reliable

---

## Current Architecture

### **Component Terminology**

| Component            | Location               | Description                                                                                                                               |
| -------------------- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| **Inference Engine** | `processor/inference/` | Python-based detection pipeline using GStreamer + Hailo SDK. Runs YOLOv8n model, produces evidence for every frame, sends via UDS.        |
| **Main Runtime**     | `processor/src/`       | Rust async runtime that spawns the Inference Engine, receives evidence via UDS, processes violations, and handles webhooks/notifications. |

### **Data Flow**

```
Camera Frame
    ↓
Inference Engine (Python)
    ├─ Run YOLOv8n detection (Hailo-8)
    ├─ Assign body parts & PPE to persons
    ├─ Mark violations per person
    └─ Send evidence for EVERY frame (regardless of violations)
    ↓
UDS (Unix Domain Socket)
    ↓
Main Runtime (Rust)
    ├─ Receive evidence JSON
    ├─ Process/aggregate violations (TODO)
    ├─ Decide when to trigger alerts (TODO)
    └─ Send webhooks to iOS (TODO)
```

**Key Design Decision:** The Inference Engine sends evidence for every frame where persons are detected, not just frames with violations. This allows the Main Runtime to:

- Aggregate violations over time (e.g., only alert if violation persists for N frames)
- Track compliance history per person
- Implement flexible alerting logic without modifying the Inference Engine

### **What We Have**

```
gidence-scm/
├── annotator/                        # Auto Annotator for automatic labeling
│   ├── model/                        # Trained Model Zoo models (.pt files)
│   ├── input/                        # Input images for auto-labeling
│   ├── output/                       # Auto-labeled annotations
│   ├── annotate.py                   # Run the auto annotator
│   └── train.ipynb                   # Notebook for training the model zoo
├── processor/                        # Main deployment package
│   ├── src/                          # Main Runtime (Rust)
│   │   ├── main.rs                   # Entry point - spawns Inference Engine, UDS listener
│   │   └── models/
│   │       ├── processor.rs          # Processor config (cameras, webhooks)
│   │       └── evidence.rs           # Evidence data structures
│   ├── inference/                    # Inference Engine (Python)
│   │   ├── main.py                   # Entry point - GStreamer callback
│   │   ├── pipeline.py               # GStreamer pipeline construction
│   │   ├── association.py            # Compliance checking logic
│   │   ├── uds.py                    # Unix Domain Socket sender
│   │   ├── model/                    # Deployment models (.hef files)
│   │   ├── so/                       # Post-processing libraries
│   │   └── core/                     # Core utilities (from hailo-apps)
│   │       └── common/               # Common utilities
│   ├── Cargo.toml                    # Rust dependencies
│   ├── pyproject.toml                # Python dependencies
│   ├── install.sh                    # Python environment setup
│   ├── setup.sh                      # Activate venv
│   ├── DEPLOY.md                     # Deployment guide
│   └── README.md                     # Processor documentation
└── CLAUDE.md                         # This file
```

### **Two-Phase Strategy**

**Phase A: Model Zoo Training (Auto-Labeling)**

```
Purpose: Generate high-quality training data
Where: Ubuntu workstation (offline)
Output: Clean, balanced dataset

Approach:
- Train 17 specialized models (one per class)
- Each model: Binary classification (object vs background)
- No class imbalance per model!
- Better bbox quality (specialized)
- Use for auto-labeling supplements

Models:
1. person-detector (foundation)
2. hardhat-detector
3. gloves-detector
4. safetyvest-detector
5. shoes-detector
6. faceguard-detector
7-17. [other classes as needed]

Inference: 17 models × 35ms = 595ms per image (acceptable for offline)
```

**Phase B: Deployment Model (Real-Time)**

```
Purpose: Real-time compliance monitoring
Where: Raspberry Pi 5 + Hailo-8
Input: High-quality data from Model Zoo
Output: Fast single model

Approach:
- Train ONE 17-class model
- Use clean data labeled by Model Zoo
- Deploy single HEF file
- Fast inference (18ms per frame on Hailo-8)

Architecture:
Stage 1: Detection (Single Model)
  - Input: Camera frame
  - Model: YOLOv8n (17 classes)
  - Output: person, body parts, PPE, tools

Stage 2: Association Logic (Python)
  - Match PPE to persons
  - Check compliance using body part visibility
  - Output: Violations per person

Multi-Camera:
  - 4 cameras simultaneously
  - Each camera independent
  - 12-15 FPS per camera on Hailo-8
```

---

## Timeline (Jan 12-24, 2026)

**Day 1-2 (Jan 12-13): Pipeline**

```
[x] Model Zoo training completes
[x] Create auto-labeling script
[ ] Auto-label supplement images
[ ] Complete human review
[ ] Train deployment model (10-class first)
[ ] Convert to HEF
[ ] Human review begins
[x] Setup Raspberry Pi + Hailo-8
[x] Implement detection pipeline
[x] Multi-camera support
[x] Association logic
```

**Day 3-5 (Jan 14-17): Integration**

```
[x] UDS output to Main Runtime
[ ] iOS notifications
[ ] Testing & refinement
```

**Day 6-12 (Jan 18-24): Documentation**

```
[ ] Portfolio documentation
[ ] Performance benchmarks
[ ] Demo videos
[ ] Polish & submit
```

**Status: 12 days remaining, ~6-8 days work needed** ✅

---

## Key Insights

### **Body Parts = Automatic Visibility**

```
Key insight: Body part detection determines when to check for PPE.

- Body part visible + PPE missing → VIOLATION
- Body part visible + PPE present → Check if properly worn
- Body part NOT visible + PPE present → Compliant (covered)
- Neither visible → Skip check (not in frame)

See "Compliance Detection Logic" below for full rules per item.
```

### **Model Zoo = Industry Standard**

```
Used by:
- COCO Dataset (annotation pipeline)
- Google Open Images (hundreds of specialized models)
- Scale AI (auto-labeling infrastructure)
- Roboflow (assisted labeling)

Not over-engineering - this is how pros do it!
```

### **Compliance Detection Logic**

Reference: `processor/inference/association.py`

```
1. Hands and Gloves:
   - Only gloves → compliant (hands covered)
   - Hand + no gloves → VIOLATION (missing_gloves)
   - Hand + gloves → VIOLATION (improperly_worn_gloves)
   - Neither → skip (not visible)

2. Foot and Shoes:
   - Only shoes → compliant
   - Foot + no shoes → VIOLATION (missing_shoes)
   - Foot + shoes → VIOLATION (improperly_worn_shoes)
   - Neither → skip

3. Ear and Earmuffs:
   - Only earmuffs → compliant
   - Ear + no earmuffs → VIOLATION (missing_earmuffs)
   - Ear + earmuffs → VIOLATION (improperly_worn_earmuffs)
   - Neither → skip

4. Face and Facemask:
   - Only facemask → compliant
   - Face + no facemask → VIOLATION (missing_facemask)
   - Face + facemask → VIOLATION (improperly_worn_facemask)
   - Neither → skip

5. Head and Hardhat (DIFFERENT):
   - Head + hardhat → compliant
   - Head + no hardhat → VIOLATION (missing_hardhat)
   - (TODO) Head + hardhat outside bbox → VIOLATION (improperly_worn_hardhat)
   - Neither → skip

6. Safetyvest (DIFFERENT):
   NOTE: Not tied to any body part detection
   - Safetyvest present → compliant
   - Safetyvest absent → VIOLATION (missing_safetyvest)

7. Glasses (NOT IMPLEMENTED):
   NOTE: Only use head if facemask is properly worn (face won't be detected)
   - (TODO) Face + glasses → compliant
   - (TODO) Face + glasses outside bbox → VIOLATION (improperly_worn_glasses)
   - (TODO) Face + no glasses → VIOLATION (missing_glasses)
```

---

## Success Criteria

**Model Zoo:** mAP > 0.50 per model (usable for auto-labeling)  
**Final Model:** mAP > 0.75 overall (production quality)  
**Deployment:** 12-15 FPS on 4 cameras (real-time)  
**Integration:** Violations → Rust → iOS (end-to-end)

**Deadline: January 24, 2026** ⭐

---

## Resources

```
Models: annotator/model (training), processor/model (deployment)
Pipeline: processor/inference/
Config: processor/config/
```
