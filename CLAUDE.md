# SCM Pipeline Development with Hailo

**Project:** Safety Compliance Monitoring (SCM) - Three-Stage PPE Detection System  
**Hardware:** Raspberry Pi 5 + Hailo-8L AI Accelerator  
**Base:** Modified from `hailo-detect-simple` example  
**Timeline:** January 6-12, 2026 (7 days)
**Repository:** gidence-scm (private)
**Working Directory:** processor/inference/

---

## Project Context

### **What We're Building**

A real-time PPE (Personal Protective Equipment) violation detection system using a three-stage hierarchical approach:

**Stage 1: Pose Detection**

- Model: YOLOv8n-pose (converted to HEF)
- Input: RTSP camera streams
- Output: Person bounding boxes + 17 COCO keypoints per person
- Purpose: Detect body parts even when covered by PPE

**Stage 2: PPE Detection**

- Model A: Common PPE (shoes, gloves, glasses, hardhat, facemask, safetyvest)
- Model B: Rare PPE (safetysuit, earmuffs, faceguard)
- Input: Same frame as Stage 1
- Output: PPE item bounding boxes with class labels
- Purpose: Detect all PPE items in frame

**Stage 3: Association Logic (Python)**

- Input: Keypoints from Stage 1 + PPE bboxes from Stage 2
- Process:
  1. Derive body part bounding boxes from keypoints
  2. Match PPE to specific body parts via IoU overlap
  3. Check compliance against requirements
- Output: Violation reports (which person missing which PPE)
- Purpose: Determine which person is missing required PPE

### **Target Deployment**

- **Cameras:** 4 RTSP streams simultaneously
- **Processing:** Real-time (12-15 FPS per camera target)
- **Output:** Violations sent to Rust runtime via UDP
- **Location:** Unilever Rungkut facility (already deployed 3-class system)

---

## Repository Structure

### **What We Have**

```
gidence-scm/                          # Project root
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

### **What Each Component Does**

**processor/:**

- Our custom SCM development area
- Contains inference pipeline implementations
- Includes core utilities needed for Hailo development
- **Where we'll build our three-stage system**

**processor/inference/core/common/:**

- Core utilities copied/adapted from Hailo frameworks
- Originally from hailo-apps repository (trimmed and customized)
- Base classes for pipeline applications
- GStreamer helpers and utilities
- Buffer management and callback infrastructure
- **We'll use and extend these for our SCM pipeline**

**Starting Point for Development:**

- We'll create our pipeline implementations in `processor/inference/`
- Reference existing detection/pose estimation examples for guidance
- Build upon the core utilities already present in the processor directory
- Focus on three-stage architecture: Pose Detection → PPE Detection → Association Logic

---

## Development Phases

### **Phase 1: Understanding (Jan 6, 2-3 hours)**

**Goal:** Understand existing core utilities and pipeline patterns

**Tasks:**

1. Examine existing code in `processor/inference/core/common/`
2. Identify key components available:
   - How pipelines can be created
   - Available base classes and utilities
   - GStreamer integration patterns
   - Metadata handling capabilities
3. Study any existing pipeline examples in the processor directory
4. Understand the Hailo inference patterns
5. Plan the three-stage architecture using available utilities

**Key Questions to Answer:**

- What base classes are available in `processor/inference/core/`?
- How do existing utilities handle GStreamer pipelines?
- What inference patterns are already implemented?
- How is metadata handled in the existing framework?
- What do we need to build from scratch vs. what's available?

**Deliverable:** Understanding of available utilities and architecture approach

---

### **Phase 2: Single Model Adaptation (Jan 7, 3-4 hours)**

**Goal:** Replace detection with pose estimation

**Tasks:**

1. Create `scm_pipeline_v1.py` in `processor/inference/`
2. Use available base classes from `processor/inference/core/common/`
3. Implement pose detection pipeline:
   - Load pose HEF model: `yolov8n-pose.hef`
   - Set up GStreamer pipeline for RTSP input
4. Modify post-processing callback:
   - Extract keypoints instead of bounding boxes
   - Parse 17 COCO keypoints per person
5. Implement `derive_body_parts()` function:
   - Head bbox from keypoints 0-4 (nose, eyes, ears)
   - Torso bbox from keypoints 5,6,11,12 (shoulders, hips)
   - Hands bbox from keypoints 9,10 (wrists)
   - Feet bbox from keypoints 15,16 (ankles)
6. Print body part bboxes to terminal (verify logic)
7. Test with camera feed

**Key Code Changes:**

```python
# In post-processing callback

# Target implementation (pose):
persons = parse_pose_detections(buffer)
for person in persons:
    keypoints = person.keypoints  # 17 points
    body_parts = derive_body_parts_from_keypoints(keypoints)
    print(f"Person detected with body parts: {body_parts.keys()}")

# Using processor/inference/core utilities
from processor.inference.core.common import ...  # Import available utilities
```

**Deliverable:** Working pose detection with body part extraction

---

### **Phase 3: Dual Model Pipeline (Jan 8, 4-5 hours)**

**Goal:** Add PPE detection model (Stage 2)

**Tasks:**

1. Create `scm_pipeline_v2.py`
2. Add second `hailonet` element:
   - First hailonet: `yolov8n-pose.hef`
   - Queue element (buffer between models)
   - Second hailonet: `common_ppe.hef`
3. Modify pipeline topology:
   ```
   source → decode → pose_net → queue → ppe_net → sink
   ```
4. Create two separate post-processing callbacks:
   - `pose_callback()`: Extract keypoints
   - `ppe_callback()`: Extract PPE bboxes
5. Store both metadata in buffer
6. Extract combined metadata in final sink
7. Test both models running sequentially
8. Measure FPS (expect ~20-25 FPS single camera)

**Pipeline Architecture:**

```
RTSP Source
    ↓
Decoder
    ↓
hailonet (pose.hef)
    ↓ [attach keypoints to buffer metadata]
hailofilter (extract keypoints)
    ↓
queue (buffer)
    ↓
hailonet (ppe.hef)
    ↓ [attach PPE detections to buffer metadata]
hailofilter (extract PPE)
    ↓
appsink (receive combined metadata)
```

**Key Challenge:** Maintaining metadata from first model while processing second model

**Deliverable:** Dual-model pipeline with both outputs

---

### **Phase 4: Association Logic (Jan 9, 4-5 hours)**

**Goal:** Implement Stage 3 (match PPE to body parts)

**Tasks:**

1. Create `association.py` module with functions:

   - `derive_body_parts_from_keypoints(keypoints) → dict`
   - `calculate_iou(bbox_a, bbox_b) → float`
   - `find_ppe_on_bodypart(bodypart_bbox, all_ppe, ppe_type) → list`
   - `check_compliance(person_data, ppe_data, requirements) → violations`

2. Implement IoU-based matching:

   ```python
   def find_ppe_on_bodypart(bodypart_bbox, all_ppe, ppe_type):
       matches = []
       for ppe in all_ppe:
           if ppe.class_name == ppe_type:
               iou = calculate_iou(bodypart_bbox, ppe.bbox)
               if iou > 0.3:  # Threshold
                   matches.append(ppe)
       return matches
   ```

3. Implement compliance checking:

   ```python
   REQUIRED_PPE = {
       'head': ['hardhat'],
       'torso': ['safetyvest'],
       'hands': ['gloves'],
       'feet': ['shoes']
   }

   def check_compliance(person, ppe_items):
       violations = []
       for body_part, required in REQUIRED_PPE.items():
           if body_part not in person.body_parts:
               continue  # Body part not visible

           for ppe_type in required:
               found = find_ppe_on_bodypart(
                   person.body_parts[body_part],
                   ppe_items,
                   ppe_type
               )
               if not found:
                   violations.append({
                       'person_id': person.id,
                       'body_part': body_part,
                       'missing_ppe': ppe_type
                   })
       return violations
   ```

4. Integrate into pipeline callback
5. Test with various scenarios:
   - Person with all PPE → no violations
   - Person missing helmet → violation detected
   - Person missing multiple items → all violations detected
   - Partially visible person → only checks visible body parts

**Deliverable:** Complete three-stage detection with violation output

---

### **Phase 5: Multi-Camera Support (Jan 10, 3-4 hours)**

**Goal:** Handle 4 RTSP streams simultaneously

**Approach:** Run 4 independent pipeline instances

**Tasks:**

1. Create `multi_camera_manager.py`:

   ```python
   class MultiCameraManager:
       def __init__(self, camera_configs):
           self.pipelines = []
           for config in camera_configs:
               pipeline = SCMPipeline(
                   camera_id=config['id'],
                   rtsp_url=config['url'],
                   pose_hef=config['pose_model'],
                   ppe_hef=config['ppe_model']
               )
               self.pipelines.append(pipeline)

       def start_all(self):
           for pipeline in self.pipelines:
               pipeline.start()

       def stop_all(self):
           for pipeline in self.pipelines:
               pipeline.stop()
   ```

2. Create `config/cameras.yaml`:

   ```yaml
   cameras:
     - id: cam_1
       name: "Entrance"
       url: "rtsp://192.168.1.101:554/stream1"
     - id: cam_2
       name: "Production Line A"
       url: "rtsp://192.168.1.102:554/stream1"
     - id: cam_3
       name: "Production Line B"
       url: "rtsp://192.168.1.103:554/stream1"
     - id: cam_4
       name: "Warehouse"
       url: "rtsp://192.168.1.104:554/stream1"

   models:
     pose: "/home/pi/optense/scm/models/hailo/yolov8n-pose.hef"
     ppe_common: "/home/pi/optense/scm/models/hailo/common_ppe.hef"
     ppe_rare: "/home/pi/optense/scm/models/hailo/rare_ppe.hef"
   ```

3. Test with all 4 cameras
4. Monitor resource usage:
   - CPU: `htop`
   - Memory: `free -h`
   - Hailo utilization: `hailortcli monitor`
5. Measure FPS per camera (target: 12-15 FPS)
6. Verify no frame drops

**Performance Expectations:**

- Single camera: 20-25 FPS
- 4 cameras: 12-15 FPS each (batch processing)
- CPU: 60-80%
- Memory: 2-3GB total
- Hailo-8L: 70-85% utilization

**Deliverable:** Multi-camera system running stably

---

### **Phase 6: Output Integration (Jan 11, 2-3 hours)**

**Goal:** Send violations to Rust runtime via UDP

**Tasks:**

1. Create `outputs/udp_sender.py`:

   ```python
   import socket
   import json

   class UDPSender:
       def __init__(self, rust_host, rust_port):
           self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
           self.address = (rust_host, rust_port)

       def send_violation(self, violation):
           message = {
               'camera_id': violation['camera_id'],
               'timestamp': violation['timestamp'],
               'person_id': violation['person_id'],
               'violations': violation['violations']
           }
           self.sock.sendto(
               json.dumps(message).encode('utf-8'),
               self.address
           )
   ```

2. Integrate into pipeline callback:

   ```python
   def on_violations_detected(self, violations):
       for violation in violations:
           self.udp_sender.send_violation({
               'camera_id': self.camera_id,
               'timestamp': time.time(),
               'person_id': violation['person_id'],
               'violations': violation['missing_ppe']
           })
   ```

3. Test with Rust receiver:

   - Rust listens on UDP port
   - Receives violation messages
   - Logs to file/database
   - Triggers iOS notifications (via APNs)

4. Verify end-to-end flow:
   - Python detects violation
   - Sends UDP message
   - Rust receives message
   - iOS app shows notification

**Message Format:**

```json
{
  "camera_id": "cam_1",
  "timestamp": 1704672345.123,
  "person_id": "person_001",
  "violations": [
    {
      "body_part": "head",
      "missing_ppe": "hardhat",
      "bbox": [100, 150, 50, 60]
    },
    {
      "body_part": "torso",
      "missing_ppe": "safetyvest",
      "bbox": [90, 200, 70, 100]
    }
  ]
}
```

**Deliverable:** Complete integration with Rust runtime

---

### **Phase 7: Testing & Optimization (Jan 12, 4-5 hours)**

**Goal:** Validate system, measure performance, document

**Tasks:**

1. **Performance Testing:**

   - Measure FPS per camera over 10 minutes
   - Monitor CPU/memory/Hailo usage
   - Test with varying number of people (1-10 per frame)
   - Measure detection accuracy on test dataset

2. **Edge Case Testing:**

   - Partially visible person (only upper body)
   - Person far from camera (small bbox)
   - Multiple people overlapping
   - Person entering/exiting frame
   - Poor lighting conditions
   - Fast-moving person (motion blur)

3. **Failure Testing:**

   - Camera disconnection (RTSP timeout)
   - Model loading failure
   - Memory overflow
   - Hailo device error
   - Network interruption

4. **Optimization:**

   - Adjust IoU thresholds for better matching
   - Tune confidence thresholds
   - Optimize frame skip rate if FPS low
   - Reduce unnecessary processing

5. **Documentation:**
   - Architecture diagram (three stages)
   - Performance benchmarks
   - Known limitations
   - Deployment guide
   - Troubleshooting guide

**Benchmarks to Record:**
| Metric | Target | Actual |
|--------|--------|--------|
| FPS per camera | 12-15 | ** |
| Detection latency | <100ms | ** |
| CPU usage | <80% | ** |
| Memory usage | <3GB | ** |
| Hailo utilization | 70-85% | ** |
| Violation detection accuracy | >85% | ** |

**Deliverable:** Validated, documented system ready for portfolio

---

## Key Implementation Details

### **GStreamer Pipeline Topology**

**Single Camera Pipeline:**

```
rtspsrc location=rtsp://camera-ip/stream
    ↓
rtph264depay
    ↓
avdec_h264
    ↓
videoconvert
    ↓
hailonet hef-path=pose.hef
    ↓
hailofilter function-name=extract_pose
    ↓
queue
    ↓
hailonet hef-path=ppe.hef
    ↓
hailofilter function-name=extract_ppe
    ↓
appsink emit-signals=true
```

### **Metadata Flow**

**Buffer Metadata Structure:**

```
GstBuffer
├─ Video frame data (raw pixels)
└─ Metadata (GstMeta)
   ├─ pose_detections: List[PersonWithKeypoints]
   │  ├─ person_id
   │  ├─ bbox
   │  └─ keypoints: 17 × (x, y, confidence)
   └─ ppe_detections: List[PPEItem]
      ├─ class_name
      ├─ bbox
      └─ confidence
```

**Accessing Metadata:**

```python
def on_buffer(self, sink, data):
    buffer = sink.emit("pull-sample").get_buffer()

    # Extract pose metadata
    pose_meta = buffer.get_meta("hailo_pose")
    persons = parse_pose_metadata(pose_meta)

    # Extract PPE metadata
    ppe_meta = buffer.get_meta("hailo_ppe")
    ppe_items = parse_ppe_metadata(ppe_meta)

    # Stage 3: Associate
    violations = self.check_compliance(persons, ppe_items)

    if violations:
        self.send_to_rust(violations)
```

### **Body Part Derivation from Keypoints**

**COCO Keypoint Indices:**

```python
KEYPOINT_INDICES = {
    'nose': 0,
    'left_eye': 1,
    'right_eye': 2,
    'left_ear': 3,
    'right_ear': 4,
    'left_shoulder': 5,
    'right_shoulder': 6,
    'left_elbow': 7,
    'right_elbow': 8,
    'left_wrist': 9,
    'right_wrist': 10,
    'left_hip': 11,
    'right_hip': 12,
    'left_knee': 13,
    'right_knee': 14,
    'left_ankle': 15,
    'right_ankle': 16
}
```

**Derivation Logic:**

```python
def derive_body_parts(keypoints):
    """
    Derive body part bounding boxes from 17 COCO keypoints.

    Args:
        keypoints: List of 17 tuples (x, y, confidence)

    Returns:
        dict: {
            'head': (x, y, w, h),
            'torso': (x, y, w, h),
            'hands': (x, y, w, h),
            'feet': (x, y, w, h)
        }
    """
    body_parts = {}

    # Head: from nose, eyes, ears (indices 0-4)
    head_points = [kp for kp in keypoints[0:5] if kp[2] > 0.5]
    if len(head_points) >= 2:
        xs = [p[0] for p in head_points]
        ys = [p[1] for p in head_points]
        margin = 30  # pixels
        body_parts['head'] = (
            min(xs) - margin,
            min(ys) - margin,
            max(xs) - min(xs) + 2*margin,
            max(ys) - min(ys) + 2*margin
        )

    # Torso: from shoulders, hips (indices 5,6,11,12)
    torso_points = [keypoints[i] for i in [5,6,11,12] if keypoints[i][2] > 0.5]
    if len(torso_points) >= 3:
        xs = [p[0] for p in torso_points]
        ys = [p[1] for p in torso_points]
        margin = 20
        body_parts['torso'] = (
            min(xs) - margin,
            min(ys) - margin,
            max(xs) - min(xs) + 2*margin,
            max(ys) - min(ys) + 2*margin
        )

    # Hands: from wrists (indices 9, 10)
    hand_points = [keypoints[i] for i in [9,10] if keypoints[i][2] > 0.5]
    if len(hand_points) >= 1:
        xs = [p[0] for p in hand_points]
        ys = [p[1] for p in hand_points]
        margin = 40
        body_parts['hands'] = (
            min(xs) - margin,
            min(ys) - margin,
            max(xs) - min(xs) + 2*margin,
            max(ys) - min(ys) + 2*margin
        )

    # Feet: from ankles (indices 15, 16)
    feet_points = [keypoints[i] for i in [15,16] if keypoints[i][2] > 0.5]
    if len(feet_points) >= 1:
        xs = [p[0] for p in feet_points]
        ys = [p[1] for p in feet_points]
        margin_h = 40
        margin_down = 60  # Extend down for shoes
        body_parts['feet'] = (
            min(xs) - margin_h,
            min(ys) - 10,
            max(xs) - min(xs) + 2*margin_h,
            max(ys) - min(ys) + margin_down
        )

    return body_parts
```

### **IoU Calculation**

```python
def calculate_iou(bbox_a, bbox_b):
    """
    Calculate Intersection over Union for two bounding boxes.

    Args:
        bbox_a: (x, y, w, h)
        bbox_b: (x, y, w, h)

    Returns:
        float: IoU value [0.0, 1.0]
    """
    x1_a, y1_a, w_a, h_a = bbox_a
    x2_a, y2_a = x1_a + w_a, y1_a + h_a

    x1_b, y1_b, w_b, h_b = bbox_b
    x2_b, y2_b = x1_b + w_b, y1_b + h_b

    # Intersection
    x1_i = max(x1_a, x1_b)
    y1_i = max(y1_a, y1_b)
    x2_i = min(x2_a, x2_b)
    y2_i = min(y2_a, y2_b)

    if x2_i < x1_i or y2_i < y1_i:
        return 0.0  # No intersection

    intersection = (x2_i - x1_i) * (y2_i - y1_i)

    # Union
    area_a = w_a * h_a
    area_b = w_b * h_b
    union = area_a + area_b - intersection

    return intersection / union if union > 0 else 0.0
```

---

## Common Challenges & Solutions

### **Challenge 1: Metadata Loss Between Models**

**Problem:** Pose detection metadata gets lost when processing second model

**Solution:** Use persistent metadata or custom GstMeta structure

```python
# Attach pose data to buffer before second model
def after_pose_detection(buffer, pose_data):
    # Create custom metadata
    meta = buffer.add_meta(CustomPoseMeta)
    meta.set_data(pose_data)
    return buffer

# Retrieve pose data after PPE detection
def after_ppe_detection(buffer, ppe_data):
    pose_meta = buffer.get_meta(CustomPoseMeta)
    pose_data = pose_meta.get_data()

    # Now have both
    violations = check_compliance(pose_data, ppe_data)
```

---

### **Challenge 2: Frame Synchronization Across Cameras**

**Problem:** Timestamps may drift between cameras

**Solution:** Use RTSP timestamps and tolerance-based matching

```python
def process_frames(self, camera_id, buffer):
    timestamp = buffer.pts  # Presentation timestamp

    # Store with timestamp
    self.frame_buffer[camera_id] = {
        'timestamp': timestamp,
        'violations': violations
    }

    # Log with consistent timing
    self.logger.info(f"[{camera_id}] Frame at {timestamp}")
```

---

### **Challenge 3: Low FPS on Multi-Camera**

**Problem:** Processing 4 cameras at 25 FPS each is too much

**Solution:** Frame skipping or lower resolution

```python
# Option 1: Process every Nth frame
self.frame_counter += 1
if self.frame_counter % 2 == 0:
    return  # Skip this frame

# Option 2: Lower input resolution
videoscale ! video/x-raw,width=1280,height=720  # Instead of 1920x1080
```

---

### **Challenge 4: Keypoints Not Detected (Occlusion)**

**Problem:** Person wearing PPE → keypoints hidden → body parts not detected

**Solution:** Use confidence threshold and fallback to person bbox

```python
def derive_body_parts(keypoints, person_bbox):
    body_parts = {}

    # Try keypoint-based detection
    head_points = [kp for kp in keypoints[0:5] if kp[2] > 0.5]

    if len(head_points) >= 2:
        # Keypoints available, use them
        body_parts['head'] = derive_from_keypoints(head_points)
    else:
        # Fallback: estimate from person bbox
        x, y, w, h = person_bbox
        body_parts['head'] = (x, y, w, h * 0.2)  # Top 20% of person

    return body_parts
```

---

## Testing Checklist

### **Functional Testing**

```
□ Single camera, pose detection only
  □ Detects people correctly
  □ Keypoints visible and accurate
  □ Body parts derived correctly

□ Single camera, pose + PPE detection
  □ Both models run sequentially
  □ Metadata from both preserved
  □ No frame drops

□ Single camera, full pipeline (3 stages)
  □ Violations detected correctly
  □ False positives < 10%
  □ False negatives < 10%

□ Multi-camera (4 streams)
  □ All cameras processing simultaneously
  □ No interference between cameras
  □ FPS meets target (12-15 per camera)

□ Output integration
  □ Violations sent to Rust via UDP
  □ Message format correct
  □ Rust receives and processes
  □ iOS notifications triggered
```

---

### **Performance Testing**

```
□ FPS measurement
  □ Single camera: 20-25 FPS
  □ Four cameras: 12-15 FPS each

□ Resource usage
  □ CPU: <80%
  □ Memory: <3GB
  □ Hailo: 70-85% utilization

□ Latency
  □ Detection latency: <100ms
  □ End-to-end (detection → notification): <500ms
```

---

### **Edge Case Testing**

```
□ Partial visibility
  □ Person partially out of frame
  □ Person occluded by object
  □ Only upper/lower body visible

□ Difficult scenarios
  □ Multiple people close together
  □ Person very far from camera
  □ Poor lighting conditions
  □ Fast-moving person

□ Failure modes
  □ Camera disconnects → pipeline restarts
  □ Model loading fails → error logged
  □ Network interruption → buffers temporarily
```

---

## Portfolio Documentation

### **What to Document**

**Architecture:**

- Three-stage pipeline diagram
- GStreamer topology visualization
- Data flow (frame → pose → PPE → violations)

**Technical Details:**

- Multi-model sequential processing
- Keypoint-based body part derivation
- IoU-based PPE matching
- Multi-camera parallel processing

**Performance:**

- FPS benchmarks (single vs multi-camera)
- Resource utilization graphs
- Detection accuracy metrics
- Comparison with baseline (3-class system)

**Challenges & Solutions:**

- Metadata preservation between models
- Body part occlusion handling
- Multi-camera synchronization
- Real-time performance optimization

**Results:**

- Before/after accuracy comparison
- Deployment at Unilever
- Real-world performance data

---

## Key Files to Create

```
processor/inference/
├── scm_pipeline_v1.py          # Phase 2: Pose detection only
├── scm_pipeline_v2.py          # Phase 3: Pose + PPE dual model
├── scm_pipeline_final.py       # Phase 4-6: Complete with Stage 3
├── multi_camera_manager.py     # Phase 5: Multi-camera handling
│
├── modules/
│   ├── association.py          # Stage 3 logic
│   ├── body_parts.py           # Keypoint → body part derivation
│   └── iou.py                  # IoU calculation
│
├── outputs/
│   └── udp_sender.py           # Send to Rust runtime
│
├── config/
│   ├── cameras.yaml            # Camera configurations
│   └── requirements.yaml       # PPE requirements
│
└── tests/
    ├── test_association.py     # Unit tests for Stage 3
    ├── test_body_parts.py      # Test body part derivation
    └── test_pipeline.py        # Integration tests
```

---

## Questions to Ask Yourself

As you develop the SCM pipeline, keep asking:

**Understanding Phase:**

- What base classes and utilities are available in `processor/inference/core/common/`?
- How are GStreamer pipelines constructed with the available utilities?
- How are callbacks registered and called?
- How is metadata attached to and extracted from buffers?
- What inference patterns can we leverage from existing code?

**Implementation Phase:**

- How do I add a second hailonet element?
- How do I preserve metadata between models?
- How do I extract both pose and PPE data?
- How do I calculate IoU efficiently?
- How do I handle missing keypoints?

**Optimization Phase:**

- What's the bottleneck (pose, PPE, or Stage 3)?
- Can I process multiple cameras truly in parallel?
- Should I skip frames or reduce resolution?
- How can I reduce false positives?

**Integration Phase:**

- How do I structure violation messages for Rust?
- How do I handle UDP send failures?
- How do I restart pipelines on camera failure?
- How do I log errors for debugging?

---

## Success Criteria

### **Phase Completion Markers**

**Phase 1 (Understanding):** ✓

- Can explain how existing utilities in `processor/inference/core/` work
- Can identify available base classes and patterns
- Understand pipeline construction and callback flow

**Phase 2 (Pose Detection):** ✓

- Pose model runs on Hailo
- Keypoints extracted correctly
- Body parts derived from keypoints

**Phase 3 (Dual Model):** ✓

- Both models run sequentially
- Both metadata types available
- FPS: 20-25 on single camera

**Phase 4 (Stage 3 Logic):** ✓

- PPE matched to body parts
- Violations detected correctly
- False positive rate < 10%

**Phase 5 (Multi-Camera):** ✓

- 4 cameras running simultaneously
- FPS: 12-15 per camera
- No crashes or frame drops

**Phase 6 (Integration):** ✓

- Violations sent to Rust via UDP
- Rust receives and processes
- iOS notifications triggered

**Phase 7 (Testing):** ✓

- All functional tests pass
- Performance meets targets
- Documentation complete

---

## Resources

### **Development Resources**

- Existing Core Utilities: `processor/inference/core/common/`
- Local processor documentation: `processor/README.md`
- Project configuration: `processor/pyproject.toml`

### **GStreamer Documentation**

- Tutorial: https://gstreamer.freedesktop.org/documentation/tutorials/
- Plugin Reference: https://gstreamer.freedesktop.org/documentation/plugins_doc.html

### **YOLO Pose**

- Ultralytics Docs: https://docs.ultralytics.com/tasks/pose/
- COCO Keypoints: https://cocodataset.org/#keypoints-2020

### **IoU Calculation**

- Explanation: https://en.wikipedia.org/wiki/Jaccard_index

---

## Timeline Summary

| Date      | Phase          | Hours           | Key Deliverable             |
| --------- | -------------- | --------------- | --------------------------- |
| Jan 6     | Understanding  | 2-3             | Example comprehension       |
| Jan 7     | Pose Detection | 3-4             | Working pose pipeline       |
| Jan 8     | Dual Model     | 4-5             | Two-model pipeline          |
| Jan 9     | Stage 3 Logic  | 4-5             | Violation detection         |
| Jan 10    | Multi-Camera   | 3-4             | 4-camera system             |
| Jan 11    | Integration    | 2-3             | End-to-end flow             |
| Jan 12    | Testing        | 4-5             | Complete documentation      |
| **Total** | **7 days**     | **22-29 hours** | **Production-ready system** |

---

## Final Notes

**Remember:**

- Start simple, add complexity incrementally
- Test after every modification
- Use existing utilities in `processor/inference/core/` as foundation
- Study the available core utilities and build upon them
- Print debug info liberally during development
- Commit working code frequently

**When stuck:**

- Re-read this document
- Study existing code in `processor/inference/core/common/` line-by-line
- Examine available utilities and base classes
- Test with minimal pipeline first
- Add one feature at a time

**You've got this!** 🚀

The example code is already 80% of what you need. Your job is understanding it and adapting it to your three-stage architecture.

**Good luck with development!**

- To give you more context, this machine that you're running on is not the target for the deployment, and the code won't run on this machine, so you don't have to run any test on this machine, I will test it manually, and report back to you
