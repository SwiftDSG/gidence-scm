"""
UDP Sender for SCM Violations

Sends violation messages to Rust runtime via UDP in JSON format.
"""

import json
import socket
from typing import Dict, List, Any

from inference.core.common.hailo_logger import get_logger

logger = get_logger(__name__)

class UDPSender:
    """
    Sends evidence messages to Rust runtime via UDP.

    Message Format:
    {
        "camera_id": "cam_1",
        "frame_id": 123,
        "timestamp": 1704672345123 # epoch time in milliseconds
        "person": [
            {
                "person_id": "person_000",
                "bbox": [100, 150, 200, 350],
                "confidence": 0.95,
                "part": [
                    {
                        "label": "hand",
                        "bbox": [120, 280, 160, 320],
                        "confidence": 0.87
                    }
                ],
                "equipment": [
                    {
                        "label": "hardhat",
                        "bbox": [108, 150, 192, 205],
                        "confidence": 0.89
                    }
                ],
                "violation": [
                    "missing_gloves"
                ]
            }
        ]
    }
    """

    def __init__(self, host: str = "127.0.0.1", port: int = 8888):
        """
        Initialize UDP sender.

        Args:
            host: Rust runtime host (default: localhost)
            port: Rust runtime port (default: 8888)
        """
        self.host = host
        self.port = port
        self.address = (host, port)

        # Create UDP socket
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

        # Statistics
        self.messages_sent = 0
        self.messages_failed = 0

        logger.info(f"UDP Sender initialized: {host}:{port}")

    def send(self, camera_id: str, frame_id: str, timestamp: int, person: List[Dict[str, Any]]) -> bool:
        """
        Send a single violation message.

        Args:
            camera_id: Camera identifier
            frame_id: Frame identifier (e.g., "frame_001")
            timestamp: Epoch time in milliseconds
            person: List of person dictionaries with keys:
                - person_id: str (e.g., "person_000")
                - bbox: List[float] [xmin, ymin, xmax, ymax]
                - confidence: float
                - part: List[Dict] (body parts)
                - equipment: List[Dict] (PPE items)
                - violation: List[str] (violation types)

        Returns:
            True if sent successfully, False otherwise
        """
        message = {
            "camera_id": camera_id,
            "frame_id": frame_id,
            "timestamp": timestamp,
            "person": person
        }

        try:
            # Serialize to JSON
            json_data = json.dumps(message)

            # Send via UDP
            self.sock.sendto(json_data.encode('utf-8'), self.address)

            self.messages_sent += 1
            logger.debug(f"Sent violation: camera={camera_id}, frame={frame_id}, person_count={len(person)}")

            return True

        except Exception as e:
            self.messages_failed += 1
            logger.error(f"Failed to send violation: {e}")
            return False

    def close(self):
        """Close the UDP socket."""
        self.sock.close()
        logger.info(f"UDP Sender closed. Stats: {self.get_stats()}")
