# SCM Project Plan

**Goal:** Finish the iOS app with full end-to-end data flow
**Last updated:** 2026-02-09

---

## Phase 1: Server — Webhook Endpoints

The processor already pushes evidence and health updates via HTTP webhooks, but the server doesn't have the routes to receive them yet. This is the single blocker for everything downstream.

- [x] 1.1 — Create `POST /evidences/{processor_id}` webhook handler
  - Receive multipart: evidence JSON + frame image from processor
  - Save image to disk, store evidence in MongoDB
  - This endpoint already exists but verify it matches what the processor sends
- [x] 1.2 — Create `POST /processors/{cluster_id}` sync webhook handler
  - Receive processor metadata + camera list
  - Upsert processor and cameras in MongoDB
  - This endpoint already exists (processor sync), verify it handles the webhook format
- [x] 1.3 — Broadcast new evidence via WebSocket
  - When a new evidence is received, push it to connected iOS clients
  - The WebSocket infra already exists in `central.rs`, just need to emit on evidence creation
- [x] 1.4 — Test server-side: processor → server → DB
  - Use the simulator to send evidence to the server
  - Verify it saves to MongoDB correctly
  - Verify the image file is saved to disk
- [x] 1.5 — Test WebSocket broadcast with iOS (after 2.0 is done)

---

## Phase 2: iOS — Remaining Core Features

With data flowing, build out the remaining screens.

- [x] 2.0 — Update iOS WebSocket handling to match new server format
  - `CentralWebSocketResponse` enum changed (renamed `Data` → `ProcessorsOnline`, `Left` → `ProcessorOffline`, added `Evidence`)
  - Update `NetworkWebSocketMessage` decoding and `ContentView.stream()` to handle the new shape
- [x] 2.1 — Evidence detail: violation breakdown
  - Show individual persons with their violations (not just count)
  - Display body parts detected, equipment present/missing
  - The data structure already supports this (`EvidencePerson` model is complete)
- [x] 2.2 — Cluster detail: uncomment and finish cameras & users sections
  - Camera list within a cluster
  - User list within a cluster
  - Both models and managers exist, just needs UI wiring
- [ ] 2.3 — (DEFERRED) Camera & processor management UI on iOS — not needed for current iteration
- [ ] 2.4 — (DEFERRED) Evidence sharing — not needed for current iteration

---

## Phase 3: Push Notifications

The subscriber system and APNS dependency exist on the server. The iOS app already handles notification permissions and token registration. Just need to trigger the actual push.

- [ ] 3.1 — Server: send APNS push when violation evidence is received
  - Look up subscribers for the relevant cluster
  - Send push via the `a2` crate (already in dependencies)
- [ ] 3.2 — iOS: handle push notification payload
  - Navigate to evidence detail when notification is tapped
  - Parse the notification payload into evidence data
- [ ] 3.3 — Test push notification end-to-end
  - Simulator → server → APNS → iOS device

---

## Phase 4: Polish & Documentation

- [ ] 4.1 — Fix subscriber refresh endpoint route bug (literal `subscriber_id` instead of `{subscriber_id}`)
- [ ] 4.2 — Error handling audit: silent failures in iOS managers
- [ ] 4.3 — Portfolio documentation
- [ ] 4.4 — Performance benchmarks
- [ ] 4.5 — Demo video

---

## Current Status

**Next action:** Start with Phase 1.1 — verify the existing evidence endpoint matches what the processor webhook sends.
