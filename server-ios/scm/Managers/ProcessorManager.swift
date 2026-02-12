//
//  ProcessorManager.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 17/10/24.
//

import SwiftUI
import Foundation
import Network
import NIO

@Observable class ProcessorManager {
    var online: [String: Int]
    
    init() {
        self.online = [:]
    }
    
    func create(_ network: Network, _ p: ProcessorRequest, f: @escaping ((Processor?, Error?) -> Void)) -> Void {
        do {
            let path = "/processors"
            
            let data = try JSONEncoder().encode(p)
            
            network.req(path, method: .post, data: data, f: f)
        } catch let error {
            print("Error: ", error)
            f(nil, error)
        }
    }
    func delete(_ network: Network,  processor_id: String, f: @escaping ((Bool?, Error?) -> Void)) -> Void {
        let path = "/processors/\(processor_id)"
        
        network.status(path, method: .delete, f: f)
    }
    func get(_ network: Network, _ processor_id: String, f: @escaping ((ViewProcessor?, Error?) -> Void)) -> Void {
        let path = "/processors/\(processor_id)"
        
        network.req(path, method: .get, f: f)
    }
    func getMany(_ network: Network, cluster_id: String? = nil, date_minimum: Int? = nil, date_maximum: Int? = nil, limit: Int? = nil, skip: Int? = nil, f: @escaping (([ViewProcessor]?, Error?) -> Void)) -> Void {
        var path = "/processors?"
        
        if let cluster_id {
            path += "cluster_id=\(cluster_id)&"
        }
        if let date_minimum {
            path += "date_minimum=\(date_minimum)&"
        }
        if let date_maximum {
            path += "date_maximum=\(date_maximum)&"
        }
        if let limit {
            path += "limit=\(limit)&"
        }
        if let skip {
            path += "skip=\(skip)&"
        }
        
        network.req(path, method: .get, f: f)
    }
}
