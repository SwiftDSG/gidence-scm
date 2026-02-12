//
//  Camera.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 03/03/25.
//

struct Camera: Hashable, Codable {
    var id: String
    var cluster_id: String
    var processor_id: String
    var address: CameraAddress
    var name: String
}

struct CameraAddress: Hashable, Codable {
    var host: [UInt8]
    var port: UInt16
    var path: String?
    var authentication: [String]?
}

struct CameraRequest: Encodable, Equatable {
    var cluster_id: String
    var processor_id: String
    var address: CameraAddress
    var name: String
}

struct CameraRef: Hashable, Codable {
    var id: String
    var name: String
}

struct ViewCamera: Hashable, Codable {
    var id: String
    var cluster: String
    var processor: String
    var address: CameraAddress
    var name: String
    var notification_count: Int
    var violation_count: Int
}
