# SCM — Safety Compliance Monitoring

**Project:** Detection-Based PPE Compliance System
**Hardware:** Raspberry Pi 5 + Hailo-8 AI Accelerator (26 TOPS)
**Approach:** Model Zoo for training, single YOLOv8n model for deployment
**Task tracking:** See `PLAN.md` for current progress and next actions

---

## System Architecture

### Components

| Component              | Location               | Tech          | Description                                                                                      |
| ---------------------- | ---------------------- | ------------- | ------------------------------------------------------------------------------------------------ |
| **Inference Engine**   | `processor/inference/` | Python        | GStreamer + Hailo SDK detection pipeline. Runs YOLOv8n, produces evidence per frame, sends via UDS. |
| **Main Runtime**       | `processor/src/`       | Rust (tokio)  | Spawns Inference Engine, receives evidence via UDS, sends webhooks to server.                     |
| **Processor Web**      | `processor-web/`       | Nuxt.js       | Local dashboard on Raspberry Pi. Camera CRUD, monitoring grid, evidence feed.                     |
| **Server**             | `server/src/`          | Rust (Actix)  | Central API. MongoDB, JWT auth, WebSocket broadcast, APNS + Telegram notifications.              |
| **Server Web**         | TBD                    | TBD           | Web version of the iOS app. Auth, clusters, evidences, user management, real-time updates.        |
| **iOS App**            | `ios/`                 | SwiftUI       | Mobile client. Clusters, evidence detail with violation breakdown, push notification handling.    |
| **Auto Annotator**     | `annotator/`           | Python        | Model Zoo training + auto-labeling for dataset generation.                                       |

### End-to-End Data Flow

```
Camera Frame
    |
Inference Engine (Python, Hailo-8)
    |- Run YOLOv8n detection
    |- Assign body parts & PPE to persons
    |- Mark violations per person
    '- Send evidence for EVERY frame with persons detected
    |
UDS (Unix Domain Socket, connection-per-message)
    |
Main Runtime (Rust)
    |- Receive evidence JSON
    |- Aggregate/deduplicate violations
    '- POST multipart webhook (JSON "data" + JPEG "image") to server
    |
Server (Actix-web)
    |- Save evidence to MongoDB + image to disk
    |- Broadcast to iOS/web clients via WebSocket
    '- Send push notifications (APNS + Telegram)
    |
    |- iOS App (real-time via WebSocket, push via APNS)
    |- Server Web (real-time via WebSocket)
    '- Telegram Bot (violation alerts to individual users)
```

### Key Design Decisions

- **UDS**: Connection-per-message pattern (not persistent). Sender closes connection after each message so receiver can use `read_to_end`.
- **Evidence for every frame**: Inference Engine sends all frames with persons, not just violations. Main Runtime decides when to alert (aggregation, persistence thresholds).
- **Normalized bounding boxes**: All bbox coordinates are 0.0-1.0, mapped to pixel space at display time.
- **Multipart webhook**: Processor sends evidence as multipart with "data" (JSON) and "image" (JPEG) fields.
- **Notification channels**: APNS for iOS push, Telegram bot for cross-platform alerts. Both triggered on violation evidence.

---

## Directory Structure

```
gidence-scm/
|- annotator/                        # Auto Annotator for automatic labeling
|   |- model/                        # Trained Model Zoo models (.pt files)
|   |- input/                        # Input images for auto-labeling
|   |- output/                       # Auto-labeled annotations
|   |- annotate.py                   # Run the auto annotator
|   '- train.ipynb                   # Notebook for training the model zoo
|- processor/                        # Deployment package (Raspberry Pi)
|   |- src/                          # Main Runtime (Rust)
|   |   |- main.rs                   # Entry point - UDS listener, webhook sender, queue processor
|   |   '- models/
|   |       |- processor.rs          # Processor config (cameras, webhooks)
|   |       '- evidence.rs           # Evidence data structures
|   |- inference/                    # Inference Engine (Python)
|   |   |- main.py                   # Entry point - GStreamer callback
|   |   |- pipeline.py               # GStreamer pipeline construction
|   |   |- association.py            # Compliance checking logic
|   |   |- uds.py                    # Unix Domain Socket sender
|   |   |- model/                    # Deployment models (.hef files)
|   |   |- so/                       # Post-processing libraries
|   |   '- core/                     # Core utilities (from hailo-apps)
|   '- routes/                       # HTTP API for processor config
|- processor-web/                    # Processor local dashboard (Nuxt.js)
|   |- app/pages/                    # Single page (index.vue - monitoring grid)
|   |- app/components/               # UI components (camera, evidence, menus)
|   '- app/composables/              # State management (camera, processor, reader)
|- server/                           # Central server (Actix-web + MongoDB)
|   '- src/
|       |- main.rs                   # Entry point - APNS thread, state manager
|       |- central.rs                # WebSocket actor + broadcast logic
|       |- database.rs               # MongoDB connection
|       |- models/                   # Data models (evidence, camera, cluster, user, etc.)
|       |- routes/                   # HTTP endpoints (evidence, camera, cluster, user, etc.)
|       '- views/                    # Response view models
|- ios/                              # iOS app (SwiftUI)
|   '- scm/
|       |- App.swift                 # AppDelegate (APNS handling)
|       |- Views/                    # UI (ContentView, EvidenceDetail, clusters, etc.)
|       |- Managers/                 # API managers (user, cluster, evidence, notification)
|       |- Models/                   # Data models
|       '- Utils/                    # Network layer (HTTP + WebSocket)
|- CLAUDE.md                         # This file - project reference
'- PLAN.md                           # Task tracking and progress
```

---

## Training Strategy

**Phase A: Model Zoo (offline, Ubuntu workstation)**
- 17 specialized single-class models for auto-labeling
- Each model: binary classification (object vs background)
- 17 models x 35ms = 595ms per image (acceptable for offline)

**Phase B: Deployment Model (real-time, Raspberry Pi)**
- Single 17-class YOLOv8n trained on Model Zoo labeled data
- Single HEF file deployed to Hailo-8
- 18ms per frame, 12-15 FPS per camera, 4 cameras simultaneously

---

## Compliance Detection Logic

Reference: `processor/inference/association.py`

```
1. Hands and Gloves:
   - Only gloves -> compliant (hands covered)
   - Hand + no gloves -> VIOLATION (missing_gloves)
   - Hand + gloves -> VIOLATION (improperly_worn_gloves)
   - Neither -> skip (not visible)

2. Foot and Shoes:
   - Only shoes -> compliant
   - Foot + no shoes -> VIOLATION (missing_shoes)
   - Foot + shoes -> VIOLATION (improperly_worn_shoes)
   - Neither -> skip

3. Ear and Earmuffs:
   - Only earmuffs -> compliant
   - Ear + no earmuffs -> VIOLATION (missing_earmuffs)
   - Ear + earmuffs -> VIOLATION (improperly_worn_earmuffs)
   - Neither -> skip

4. Face and Facemask:
   - Only facemask -> compliant
   - Face + no facemask -> VIOLATION (missing_facemask)
   - Face + facemask -> VIOLATION (improperly_worn_facemask)
   - Neither -> skip

5. Head and Hardhat (DIFFERENT):
   - Head + hardhat -> compliant
   - Head + no hardhat -> VIOLATION (missing_hardhat)
   - Neither -> skip

6. Safetyvest (DIFFERENT):
   NOTE: Not tied to any body part detection
   - Safetyvest present -> compliant
   - Safetyvest absent -> VIOLATION (missing_safetyvest)
```

---

## Success Criteria

**Model Zoo:** mAP > 0.50 per model (usable for auto-labeling)
**Final Model:** mAP > 0.75 overall (production quality)
**Deployment:** 12-15 FPS on 4 cameras (real-time)
**Integration:** Violations -> Rust -> Server -> iOS/Web/Telegram (end-to-end)
