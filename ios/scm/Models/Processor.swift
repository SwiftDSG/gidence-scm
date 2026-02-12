//
//  Processor.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 17/10/24.
//

struct Processor: Hashable, Codable {
    var id: String
    var cluster_id: String
    var address: ProcessorAddress
    var name: String
    var version: String
}

struct ProcessorAddress: Hashable, Codable {
    var host: [UInt8]
    var port: UInt16
}

struct ProcessorRequest: Encodable, Equatable {
    var id: String
    var cluster_id: String
    var address: ProcessorAddress
    var name: String
}

struct ProcessorRef: Hashable, Codable {
    var id: String
    var name: String
}

struct ViewProcessor: Hashable, Codable {
    var id: String
    var cluster: CameraRef
    var camera: [ViewCamera]
    var address: ProcessorAddress
    var name: String
    var violation_count: Int
    var notification_count: Int
}
