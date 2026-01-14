use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Evidence {
    // When parsing, if the id isn't present, fill with empty string
    #[serde(default)]
    pub id: String,
    pub camera_id: String,
    pub frame_id: String,
    pub timestamp: i64,
    pub person: EvidencePerson,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidencePerson {
    pub id: String,
    pub bbox: [f32; 4],
    pub confidence: f32,
    pub part: Vec<EvidencePersonPart>,
    pub equipment: Vec<EvidencePersonEquipment>,
    pub violation: Vec<EvidencePersonViolation>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidencePersonPart {
    pub label: EvidencePersonPartLabel,
    pub bbox: [f32; 4],
    pub confidence: f32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidencePersonEquipment {
    pub label: EvidencePersonEquipmentLabel,
    pub bbox: [f32; 4],
    pub confidence: f32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EvidencePersonPartLabel {
    Head,
    Hand,
    Face,
    Foot,
    Ear,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EvidencePersonEquipmentLabel {
    Hardhat,
    Gloves,
    Shoes,
    Safetyvest,
    Safetysuit,
    Facemask,
    Faceguard,
    Earmuffs,
    Glasses,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePersonViolation {
    MissingHardhat,
    MissingGloves,
    MissingShoes,
    MissingFacemask,
    MissingEarmuffs,
    MissingSafetyvest,
    ImproperlyWornGloves,
    ImproperlyWornShoes,
    ImproperlyWornFacemask,
    ImproperlyWornEarmuffs,
}
