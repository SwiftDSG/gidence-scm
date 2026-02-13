export type Evidence = {
  id: string;
  camera_id: string;
  frame_id: string;
  timestamp: number;
  person: EvidencePerson[];
};

export type EvidencePerson = {
  id: string;
  bbox: [number, number, number, number];
  confidence: number;
  part: EvidencePersonPart[];
  equipment: EvidencePersonEquipment[];
  violation: EvidencePersonViolation[];
};

export type EvidencePersonPart = {
  label: EvidencePersonPartLabel;
  bbox: [number, number, number, number];
  confidence: number;
};

export type EvidencePersonEquipment = {
  label: EvidencePersonEquipmentLabel;
  bbox: [number, number, number, number];
  confidence: number;
};

export type EvidencePersonPartLabel =
  | "head"
  | "hand"
  | "face"
  | "foot"
  | "ear";

export type EvidencePersonEquipmentLabel =
  | "hardhat"
  | "gloves"
  | "shoes"
  | "safetyvest"
  | "safetysuit"
  | "facemask"
  | "faceguard"
  | "earmuffs"
  | "glasses";

export type EvidencePersonViolation =
  | "missing_hardhat"
  | "missing_gloves"
  | "missing_shoes"
  | "missing_facemask"
  | "missing_earmuffs"
  | "missing_safetyvest"
  | "improperly_worn_hardhat"
  | "improperly_worn_gloves"
  | "improperly_worn_shoes"
  | "improperly_worn_facemask"
  | "improperly_worn_earmuffs";
