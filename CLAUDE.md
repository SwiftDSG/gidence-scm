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

### **What We Have**

```
gidence-scm/                          # Project root
├── annotator/                        # Auto Annotator for automatic labeling
│   ├── model/                        # Models for all the labeling via auto annotator
│   ├── annotate.py                   # Run the auto annotator
│   └── train.ipynb                   # Notebook for training the model zoo
├── processor/                        # Custom SCM processor (our development)
│   ├── inference/                    # Inference pipeline implementations
│   │   ├── core/                     # Core utilities (copied from hailo-apps)
│   │   │   └── common/               # Common utilities
│   │   └── config/                   # Configuration files
│   ├── install.sh                    # Installation script
│   ├── pyproject.toml                # Python project configuration
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
□ Model Zoo training completes
□ Create auto-labeling script
□ Auto-label supplement images
□ Complete human review
□ Train deployment model (17-class)
□ Convert to HEF
□ Human review begins
□ Setup Raspberry Pi + Hailo-8
□ Implement detection pipeline
□ Multi-camera support
□ Association logic
```

**Day 3-5 (Jan 14-17): Integration**

```
□ UDP output to Rust
□ iOS notifications
□ Testing & refinement
```

**Day 6-12 (Jan 18-24): Documentation**

```
□ Portfolio documentation
□ Performance benchmarks
□ Demo videos
□ Polish & submit
```

**Status: 12 days remaining, ~6-8 days work needed** ✅

---

## Key Insights

### **Body Parts = Automatic Visibility**

```
Model behavior:
- If 'hand' detected → bare hands visible → check gloves
- If 'gloves' detected → hands covered → compliant!
- If neither → hands not visible → skip check

This solves the visibility problem elegantly!
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