//
//  Evidence.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 05/02/26.
//

struct Evidence: Codable, Identifiable {
    var id: String
    var cluster_id: String
    var processor_id: String
    var camera_id: String
    var frame_id: String
    var timestamp: Int64
    var person: [EvidencePerson]
}

struct EvidencePerson: Hashable, Codable {
    var id: String
    var bbox: [Float]
    var confidence: Float
    var part: [EvidencePersonPart]
    var equipment: [EvidencePersonEquipment]
    var violation: [EvidencePersonViolation]
}

struct EvidencePersonPart: Hashable, Codable {
    var label: EvidencePersonPartLabel
    var bbox: [Float]
    var confidence: Float
}
enum EvidencePersonPartLabel: String, Equatable, Codable {
    case head
    case hand
    case face
    case foot
    case ear
}

struct EvidencePersonEquipment: Hashable, Codable {
    var label: EvidencePersonEquipmentLabel
    var bbox: [Float]
    var confidence: Float
}
enum EvidencePersonEquipmentLabel: String, Equatable, Codable {
    case hardhat
    case gloves
    case shoes
    case safetyvest
    case safetysuit
    case facemask
    case faceguard
    case earmuffs
    case glasses
}

enum EvidencePersonViolation: String, Equatable, Codable {
    case missing_hardhat
    case missing_gloves
    case missing_shoes
    case missing_facemask
    case missing_earmuffs
    case missing_safetyvest
    case improperly_worn_hardhat
    case improperly_worn_gloves
    case improperly_worn_shoes
    case improperly_worn_facemask
    case improperly_worn_earmuffs
}

struct ViewEvidence: Hashable, Codable, Identifiable {
    var id: String
    var cluster: ClusterRef
    var processor: ProcessorRef
    var camera: CameraRef
    var timestamp: Int64
    var person: [EvidencePerson]
}
