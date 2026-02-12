#!/usr/bin/env python3
"""
Inference Engine Simulator

Simulates the Inference Engine for development without Raspberry Pi + Hailo-8L.
Sends evidence via UDS with configurable violation states.

Usage:
    python -m simulator.main
    python -m simulator.main --fps 2 --camera cam_test
"""

import argparse
import json
import os
import shutil
import socket
import sys
import termios
import time
import tty
from pathlib import Path
from threading import Thread, Event
from typing import List, Optional


# Evidence format matching the real Inference Engine
def create_evidence(
    camera_id: str,
    frame_id: str,
    timestamp: int,
    persons: List[dict]
) -> dict:
    """Create evidence JSON matching the Inference Engine format."""
    return {
        "camera_id": camera_id,
        "frame_id": frame_id,
        "timestamp": timestamp,
        "person": persons
    }


def create_person(
    person_id: str,
    bbox: List[float],
    confidence: float,
    parts: List[dict],
    equipment: List[dict],
    violations: List[str]
) -> dict:
    """Create a person detection entry."""
    return {
        "id": person_id,
        "bbox": bbox,
        "confidence": confidence,
        "part": parts,
        "equipment": equipment,
        "violation": violations
    }


class UDSSender:
    """Unix Domain Socket sender for evidence."""

    def __init__(self, path: str = "/tmp/gidence-scm_uds.sock"):
        self.path = path
        self.sock: Optional[socket.socket] = None

    def connect(self) -> bool:
        """Connect to the UDS socket."""
        try:
            if self.sock:
                self.sock.close()
            self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
            self.sock.connect(self.path)
            return True
        except (socket.error, FileNotFoundError) as e:
            self.sock = None
            return False

    def send(self, evidence: dict) -> bool:
        """Send evidence JSON via UDS."""
        if not self.sock:
            if not self.connect():
                return False

        try:
            payload = json.dumps(evidence).encode('utf-8')
            self.sock.sendall(payload)
            self.sock.close()
            return True
        except (socket.error, BrokenPipeError):
            self.sock = None
            return False

    def close(self):
        """Close the socket connection."""
        if self.sock:
            self.sock.close()
            self.sock = None


class InferenceSimulator:
    """Simulates the Inference Engine for development."""

    # Static 3 persons with different positions
    PERSONS_CONFIG = [
        {
            "id": "001",
            "bbox": [0.2395833333, 0.2398148148, 0.309375, 0.4037037037],
            "confidence": 0.95,
            "parts": [
                {"label": "head", "bbox": [0.240625, 0.2398148148, 0.26875, 0.3018518519] , "confidence": 0.92},
                {"label": "hand", "bbox": [0.2401041667, 0.3490740741, 0.2557291667, 0.3740740741] , "confidence": 0.92},
                {"label": "hand", "bbox": [0.2630208333, 0.3425925926, 0.2807291667, 0.3694444444], "confidence": 0.88},
            ],
            "equipment_normal": [
                {"label": "hardhat", "bbox": [0.240625, 0.2398148148, 0.2677083333, 0.2805555556], "confidence": 0.91},
                {"label": "facemask", "bbox": [0.2473958333, 0.275, 0.2671875, 0.3018518519], "confidence": 0.91},
                {"label": "safetyvest", "bbox": [0.2427083333, 0.2731481481, 0.3, 0.3453703704], "confidence": 0.93},
            ],
            "equipment_violation": [
                {"label": "hardhat", "bbox": [0.240625, 0.2398148148, 0.2677083333, 0.2805555556], "confidence": 0.91},
                {"label": "facemask", "bbox": [0.2473958333, 0.275, 0.2671875, 0.3018518519], "confidence": 0.91},
                # Missing gloves
                {"label": "safetyvest", "bbox": [0.2427083333, 0.2731481481, 0.3, 0.3453703704], "confidence": 0.93},
            ],
            "violations": ["missing_gloves"],
        },
        {
            "id": "002",
            "bbox": [0.4015625, 0.0601851852, 0.4505208333, 0.387962963],
            "confidence": 0.92,
            "parts": [
                {"label": "head", "bbox": [0.415625, 0.062962963, 0.4416666667, 0.1185185185], "confidence": 0.90},
                {"label": "face", "bbox": [0.4255208333, 0.0898148148, 0.4395833333, 0.1185185185], "confidence": 0.90},
                {"label": "hand", "bbox": [0.4276041667, 0.1148148148, 0.4416666667, 0.1472222222], "confidence": 0.85},
            ],
            "equipment_normal": [
                {"label": "hardhat", "bbox": [0.4135416667, 0.0601851852, 0.4453125, 0.0981481481], "confidence": 0.89},
                {"label": "safetyvest", "bbox": [0.4010416667, 0.1148148148, 0.4515625, 0.2351851852], "confidence": 0.91},
            ],
            "equipment_violation": [
                # Missing hardhat
                {"label": "hardhat", "bbox": [0.4135416667, 0.0601851852, 0.4453125, 0.0981481481], "confidence": 0.89},
                {"label": "safetyvest", "bbox": [0.4010416667, 0.1148148148, 0.4515625, 0.2351851852], "confidence": 0.91},
            ],
            "violations": ["missing_gloves", "missing_facemask"],
        },
        {
            "id": "003",
            "bbox": [0.540625, 0.1166666667, 0.5973958333, 0.4268518519],
            "confidence": 0.88,
            "parts": [
                {"label": "head", "bbox": [0.5604166667, 0.1166666667, 0.5921875, 0.1722222222], "confidence": 0.87},
                {"label": "hand", "bbox": [0.55625, 0.2953703704, 0.5708333333, 0.3259259259], "confidence": 0.87},
            ],
            "equipment_normal": [
                {"label": "hardhat", "bbox": [0.540625, 0.1611111111, 0.5729166667, 0.2203703704], "confidence": 0.85},
                {"label": "safetyvest", "bbox": [0.5546875, 0.1657407407, 0.5989583333, 0.3037037037], "confidence": 0.89},
            ],
            "equipment_violation": [
                {"label": "hardhat", "bbox": [0.540625, 0.1611111111, 0.5729166667, 0.2203703704], "confidence": 0.85},
                {"label": "safetyvest", "bbox": [0.5546875, 0.1657407407, 0.5989583333, 0.3037037037], "confidence": 0.89},
            ],
            "violations": ["missing_gloves", "improperly_worn_hardhat"],
        },
    ]

    def __init__(self, camera_id: str = "cam_sim", fps: float = 1.0):
        self.camera_id = camera_id
        self.fps = fps
        self.frame_interval = 1.0 / fps

        self.frame_count = 0
        self.violation_mode = False
        self.running = False
        self.stop_event = Event()

        self.uds = UDSSender()
        self.send_success = 0
        self.send_failed = 0

        # Image paths
        self.simulator_dir = Path(__file__).parent
        self.images_dir = self.simulator_dir / "images"
        self.normal_image = self.images_dir / "normal.jpg"
        self.violation_image = self.images_dir / "violation.jpg"

        # Evidence output directory
        self.evidence_dir = Path("/tmp")
        self.evidence_dir.mkdir(parents=True, exist_ok=True)

    def get_persons(self) -> List[dict]:
        """Get person detections based on current mode."""
        persons = []

        for config in self.PERSONS_CONFIG:
            if self.violation_mode:
                equipment = config["equipment_violation"]
                violations = config["violations"]
            else:
                equipment = config["equipment_normal"]
                violations = []

            person = create_person(
                person_id=config["id"],
                bbox=config["bbox"],
                confidence=config["confidence"],
                parts=config["parts"],
                equipment=equipment,
                violations=violations
            )
            persons.append(person)

        return persons

    def get_current_image(self) -> Optional[Path]:
        """Get the image path for current mode."""
        if self.violation_mode:
            return self.violation_image if self.violation_image.exists() else None
        else:
            return self.normal_image if self.normal_image.exists() else None

    def save_evidence_image(self, frame_id: str, timestamp: int):
        """Overwrite the appropriate image to evidence directory."""
        src_image = self.get_current_image()
        if src_image:
            dst_path = self.evidence_dir / f"{self.camera_id}.jpg"
            shutil.copy(src_image, dst_path)
            return dst_path
        return None

    def send_frame(self) -> bool:
        """Send a single frame of evidence."""
        self.frame_count += 1
        frame_id = f"{self.frame_count:06d}"
        timestamp = int(time.time() * 1000)

        persons = self.get_persons()
        evidence = create_evidence(
            camera_id=self.camera_id,
            frame_id=frame_id,
            timestamp=timestamp,
            persons=persons
        )

        # Save evidence image
        self.save_evidence_image(frame_id, timestamp)

        # Send via UDS
        success = self.uds.send(evidence)
        if success:
            self.send_success += 1
        else:
            self.send_failed += 1

        return success

    def frame_loop(self):
        """Background thread for sending frames."""
        while not self.stop_event.is_set():
            self.send_frame()
            self.stop_event.wait(self.frame_interval)

    def toggle_violation(self):
        """Toggle violation mode."""
        self.violation_mode = not self.violation_mode

    def get_status_display(self) -> str:
        """Get the current status display string."""
        mode = "VIOLATION" if self.violation_mode else "NORMAL"
        mode_color = "\033[91m" if self.violation_mode else "\033[92m"  # Red or Green
        reset = "\033[0m"

        # Count violations
        if self.violation_mode:
            violation_count = sum(len(p["violations"]) for p in self.PERSONS_CONFIG)
            violations_str = ", ".join(
                p["violations"][0] for p in self.PERSONS_CONFIG if p["violations"]
            )
        else:
            violation_count = 0
            violations_str = "(none)"

        # Connection status
        conn_status = "\033[92m connected\033[0m" if self.uds.sock else "\033[91m disconnected\033[0m"

        # Build display
        lines = [
            "",
            "\033[2J\033[H",  # Clear screen
            "=" * 60,
            "  INFERENCE ENGINE SIMULATOR",
            "=" * 60,
            "",
            f"  Status: {mode_color}{mode}{reset}",
            f"  Frame: {self.frame_count:06d} | FPS: {self.fps} | Persons: {len(self.PERSONS_CONFIG)}",
            f"  UDS: {conn_status} | Sent: {self.send_success} | Failed: {self.send_failed}",
            "",
            "  Controls:",
            "    [v] Toggle violation mode",
            "    [q] Quit",
            "",
            f"  Current violations ({violation_count}): {violations_str}",
            "",
            "=" * 60,
        ]

        # Check for images
        if not self.normal_image.exists():
            lines.append(f"  \033[93mWARNING: {self.normal_image} not found\033[0m")
        if not self.violation_image.exists():
            lines.append(f"  \033[93mWARNING: {self.violation_image} not found\033[0m")

        return "\n".join(lines)

    def run(self):
        """Run the simulator with interactive CLI."""
        self.running = True

        # Start frame sending thread
        frame_thread = Thread(target=self.frame_loop, daemon=True)
        frame_thread.start()

        # Save terminal settings
        old_settings = termios.tcgetattr(sys.stdin)

        try:
            # Set terminal to raw mode for single key input
            tty.setcbreak(sys.stdin.fileno())

            print(self.get_status_display())

            while self.running:
                # Check for keyboard input
                if self._kbhit():
                    key = sys.stdin.read(1).lower()

                    if key == 'q':
                        self.running = False
                        break
                    elif key == 'v':
                        self.toggle_violation()

                # Update display
                print(self.get_status_display())
                time.sleep(0.1)

        finally:
            # Restore terminal settings
            termios.tcsetattr(sys.stdin, termios.TCSADRAIN, old_settings)
            self.stop_event.set()
            self.uds.close()
            print("\n\nSimulator stopped.")

    def _kbhit(self) -> bool:
        """Check if a key has been pressed."""
        import select
        return select.select([sys.stdin], [], [], 0)[0] != []


def main():
    parser = argparse.ArgumentParser(description='Inference Engine Simulator')
    parser.add_argument('--fps', type=float, default=1.0,
                        help='Frames per second to send (default: 1.0)')
    parser.add_argument('--camera', type=str, default='cam_sim',
                        help='Camera ID to use (default: cam_sim)')

    args = parser.parse_args()

    print("Starting Inference Engine Simulator...")
    print(f"Camera: {args.camera} | FPS: {args.fps}")
    print("Waiting for Main Runtime to start (UDS socket)...")
    print("Press Ctrl+C to cancel\n")

    simulator = InferenceSimulator(
        camera_id=args.camera,
        fps=args.fps
    )

    try:
        simulator.run()
    except KeyboardInterrupt:
        print("\n\nInterrupted by user.")


if __name__ == "__main__":
    main()
