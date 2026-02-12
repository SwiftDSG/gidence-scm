//
//  Cluster.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 17/10/24.
//
struct Cluster: Hashable, Codable {
    var id: String
    var name: String
}

struct ClusterRequest: Hashable, Codable {
    var name: String
}

struct ClusterRef: Hashable, Codable {
    var id: String
    var name: String
}

struct ViewCluster: Hashable, Codable, Identifiable {
    var id: String
    var name: String
    var processor_count: Int
    var violation_count: Int
    var notification_count: Int
}
