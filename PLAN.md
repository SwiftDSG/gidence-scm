# SCM Project Plan

**Goal:** Finish iOS app, processor web, server web, Telegram notifications, and benchmarks
**Last updated:** 2026-02-12

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

- [x] 3.1 — Server: send APNS push when violation evidence is received
  - Look up subscribers for the relevant cluster
  - Send push via the `a2` crate (already in dependencies)
- [x] 3.2 — iOS: handle push notification payload
  - Navigate to evidence detail when notification is tapped
  - Parse the notification payload into evidence data
- [ ] 3.3 — Test push notification end-to-end
  - Simulator → server → APNS → iOS device

---

## Phase 4: Processor Web Interface

Nuxt.js dashboard running locally on the Raspberry Pi. Sidebar handles processor info editing and camera CRUD. Home page is a monitoring view.

- [x] 4.1 — Camera CRUD in sidebar
  - Add camera: form with name, RTSP URL
  - Edit camera: update name, RTSP URL
  - Delete camera: with confirmation
  - Wire to existing processor API (`POST/PUT/DELETE /camera`)
  - Added `GET /camera/{id}/frame` endpoint for serving latest frame images
- [x] 4.2 — Home page: camera monitoring grid
  - Grid of camera cards with latest frame (cache-busted on evidence update)
  - FPS metric per camera (calculated from UDS message timestamps in Reading struct)
  - Bounding box overlays with violation-aware coloring (person=white, parts/equipment=green/red)
- [ ] 4.3 — Evidence feed per camera
  - Show list of recent evidences for the selected camera
  - Each evidence shows: frame thumbnail, timestamp, violation count
  - Basic violation summary (which violations were detected)

---

## Phase 5: Server Web Interface

Web version of the iOS app — accessible from any device via browser. Same features: auth, clusters, evidences with violation breakdown, user management, real-time updates.

- [ ] 5.1 — Auth: login page
  - Login form (email/password)
  - Token storage + auto-refresh
  - Protected routes (redirect to login if unauthenticated)
- [ ] 5.2 — Cluster list + detail page
  - List all clusters the user belongs to
  - Cluster detail: cameras list, users list, processor status (online/offline)
- [ ] 5.3 — Evidence list + detail page
  - Filterable list (by cluster, camera, date range)
  - Detail page: image viewer with person bounding boxes, violation breakdown per person
  - Same violation coloring logic as iOS (body parts blue/red, equipment green/red)
- [ ] 5.4 — User management page
  - List users in a cluster
  - Create / edit / delete users
- [ ] 5.5 — Real-time updates via WebSocket
  - Live evidence feed (new evidences appear without refresh)
  - Processor online/offline status
- [ ] 5.6 — Notification settings
  - Subscribe / unsubscribe from push notifications
  - Toggle notification preferences

---

## Phase 6: Telegram Notifications

Additional notification channel alongside APNS. Server sends violation alerts to individual users via a Telegram bot.

- [ ] 6.1 — Server: Telegram bot integration
  - Create bot via BotFather, store bot token in server config
  - Add Telegram send logic alongside existing APNS notification thread
  - When violation evidence is received, send to both APNS subscribers AND Telegram subscribers
- [ ] 6.2 — Subscriber model: support Telegram
  - Extend subscriber `kind` to support `telegram` with chat ID (alongside existing `apple` with device token)
  - Update `POST /subscribers` and `DELETE /subscribers` to handle Telegram kind
- [ ] 6.3 — User linking via bot
  - User sends `/start` to the bot with a linking code (generated from the app)
  - Bot registers the user's Telegram chat ID as a subscriber
  - Linking code ties the chat ID to the correct user + cluster

---

## Phase 7: Benchmarks & Polish

- [x] 7.1 — Fix subscriber refresh endpoint route bug (literal `subscriber_id` instead of `{subscriber_id}`)
- [x] 7.2 — Error handling audit: silent failures in iOS managers
  - Added generic `req<T>` and `status` helpers on Network to centralize error handling
  - Refactored all managers to use the new helpers
  - Fixed double-callback bug in NotificationManager (subscribe/refresh)
  - Fixed Bool vs Bool? type mismatch in delete/unsubscribe callbacks
- [ ] 7.3 — Test push notification end-to-end
  - Simulator → server → APNS → iOS device
- [ ] 7.4 — Performance benchmarks
- [ ] 7.5 — Portfolio documentation
- [ ] 7.6 — Demo video

---

## Current Status

**Next action:** Phase 4.1 — Camera CRUD in the processor web sidebar.
