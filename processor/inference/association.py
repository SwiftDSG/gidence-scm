"""
Association Logic for SCM Detection

Returns complete frame detections with violation markers for frontend control.

Data Structure (simplified):
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

No redundant properties:
- has_violations can be inferred from violations list
- severity can be inferred from type
- affected_detections are can be inferred from the violation kind
"""

from typing import Dict, List, Any

from inference.core.common.hailo_logger import get_logger

logger = get_logger(__name__)

# Body part detection classes
BODY_PARTS = {"head", "hand", "foot", "face", "ear"}

# PPE item classes
PPE_ITEMS = {
    "hardhat",
    "gloves",
    "shoes",
    "safetyvest",
    "safetysuit",
    "faceguard",
    "facemask",
    "earmuffs",
    "glasses",
}

VIOLATION_KINDS = {
    "missing_hardhat",
    "missing_gloves",
    "missing_shoes",
    "missing_facemask",
    "missing_earmuffs",
    "improperly_worn_gloves",
    "improperly_worn_shoes",
    "improperly_worn_facemask",
    "improperly_worn_earmuffs",
}


def is_contained(inner_bbox: Dict, outer_bbox: Dict, margin: float = 0.05) -> bool:
    """Check if inner_bbox is contained within outer_bbox (with margin)."""
    width = outer_bbox.xmax() - outer_bbox.xmin()
    height = outer_bbox.ymax() - outer_bbox.ymin()

    expanded_xmin = outer_bbox.xmin() - margin * width
    expanded_ymin = outer_bbox.ymin() - margin * height
    expanded_xmax = outer_bbox.xmax() + margin * width
    expanded_ymax = outer_bbox.ymax() + margin * height

    return (
        inner_bbox.xmin() >= expanded_xmin
        and inner_bbox.ymin() >= expanded_ymin
        and inner_bbox.xmax() <= expanded_xmax
        and inner_bbox.ymax() <= expanded_ymax
    )


def bbox_to_list(bbox) -> List[float]:
    """Convert Hailo bbox to list [xmin, ymin, xmax, ymax]."""
    return [bbox.xmin(), bbox.ymin(), bbox.xmax(), bbox.ymax()]


def assign_detections_to_persons(
    persons: List[Dict], others: List[Dict]
) -> Dict[int, Dict[str, Any]]:
    """
    Assign body parts and PPE items to each person based on containment.

    Returns:
        Dictionary mapping person_index -> {
            'person': person_detection,
            'part': [list of body part detections],
            'equipment': [list of PPE item detections]
        }
    """
    person_assignments = {}

    for i, person in enumerate(persons):
        person_bbox = person["bbox"]

        part = []
        equipment = []

        for detection in others:
            det_bbox = detection["bbox"]
            det_label = detection["label"]

            if is_contained(det_bbox, person_bbox, margin=0.05):
                if det_label in BODY_PARTS:
                    part.append(detection)
                elif det_label in PPE_ITEMS:
                    equipment.append(detection)

        person_assignments[i] = {
            "person": person,
            "part": part,
            "equipment": equipment,
        }

    return person_assignments


def check_compliance_for_person(
    assignment: Dict[str, Any]
) -> List[str]:
    """
    Check PPE compliance for a single person.

    Detection Logic:

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
       - (TODO) Head + hardhat (outside head bbox) → VIOLATION (improperly_worn_hardhat)
       - Neither → skip

    6. Face / Head (if facemask is present) and Glasses (NOT IMPLEMENTED):
       NOTE: Only use head if facemask is properly worn (face won't be detected)
       - (TODO) Face + glasses → compliant
       - (TODO) Face + glasses (outside face bbox) → VIOLATION (improperly_worn_glasses)
       - (TODO) Face + no glasses → VIOLATION (missing_glasses)

    Returns:
        List[str]
    """
    violations = []

    part = assignment["part"]
    equipment = assignment["equipment"]

    # Create lookup dictionaries
    part_labels = [item["label"] for item in part]
    equipment_labels = [item["label"] for item in equipment]

    # ========== Logic 1: Hands and Gloves ==========
    has_hand = "hand" in part_labels
    has_gloves = "gloves" in equipment_labels

    if has_hand and has_gloves:
        violations.append("improperly_worn_gloves")

    elif has_hand and not has_gloves:
        violations.append("missing_gloves")

    # ========== Logic 2: Foot and Shoes ==========
    has_foot = "foot" in part_labels
    has_shoes = "shoes" in equipment_labels

    if has_foot and has_shoes:
        violations.append("improperly_worn_shoes")

    elif has_foot and not has_shoes:
        violations.append("missing_shoes")

    # ========== Logic 3: Ear and Earmuffs ==========
    has_ear = "ear" in part_labels
    has_earmuffs = "earmuffs" in equipment_labels

    if has_ear and has_earmuffs:
        violations.append("improperly_worn_earmuffs")

    elif has_ear and not has_earmuffs:
        violations.append("missing_earmuffs")

    # ========== Logic 4: Face and Facemask ==========
    has_face = "face" in part_labels
    has_facemask = "facemask" in equipment_labels

    if has_face and has_facemask:
        violations.append("improperly_worn_facemask")

    elif has_face and not has_facemask:
        violations.append("missing_facemask")

    # ========== Logic 5: Head and Hardhat (DIFFERENT) ==========
    has_head = "head" in part_labels
    has_hardhat = "hardhat" in equipment_labels

    if has_head and has_hardhat:
        # Compliant - head covered with hardhat
        pass

    elif has_head and not has_hardhat:
        violations.append("missing_hardhat")

    elif not has_head and has_hardhat:
        # Hardhat present but head not visible - treat as compliant
        pass

    return violations


def check_compliance_all_persons(
    person_assignments: Dict[int, Dict[str, Any]]
) -> List[Dict[str, Any]]:
    """
    Check compliance for all persons and return complete frame snapshot.

    Returns:
        [
            {
                "person_id": "person_000",
                "bbox": [100, 150, 200, 350],
                "confidence": 0.95,
                "part": [...],
                "equipment": [...],
                "violation": [...]
            }
        ]
    """

    persons = []

    for i, assignment in person_assignments.items():
        person = assignment["person"]
        person_id = person["person_id"]

        # Check compliance
        violations = check_compliance_for_person(assignment)

        # Convert bboxes to lists
        person_bbox = bbox_to_list(person["bbox"])
        for part in assignment["part"]:
            part["bbox"] = bbox_to_list(part["bbox"])
        for equipment in assignment["equipment"]:
            equipment["bbox"] = bbox_to_list(equipment["bbox"])

        # Build person data
        person_data = {
            "person_id": person_id,
            "bbox": person_bbox,
            "confidence": person["confidence"],
            "part": assignment["part"],
            "equipment": assignment["equipment"],
            "violation": violations
        }

        persons.append(person_data)

        if violations:
            logger.info(f"Person {person_id}: {len(violations)} violation(s)")

    return persons
