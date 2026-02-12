//
//  ClusterManager.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 17/10/24.
//

import SwiftUI

@Observable class ClusterManager {
    func create(_ network: Network, p: ClusterRequest, f: @escaping ((ViewCluster?, Error?) -> Void)) -> Void {
        do {
            let path = "/clusters"
            
            let data = try JSONEncoder().encode(p)
            
            network.req(path, method: .post, data: data, f: f)
        } catch let error {
            print("Error: ", error)
            f(nil, error)
        }
    }
    func get(_ network: Network, cluster_id: String, date_minimum: Int? = nil, date_maximum: Int? = nil, f: @escaping ((ViewCluster?, Error?) -> Void)) -> Void {
        var path = "/clusters/\(cluster_id)?"
        
        if let date_minimum {
            path += "date_minimum=\(date_minimum)&"
        }
        if let date_maximum {
            path += "date_maximum=\(date_maximum)&"
        }
        
        network.req(path, method: .get, f: f)
    }
    func getMany(_ network: Network, date_minimum: Int? = nil, date_maximum: Int? = nil, f: @escaping (([ViewCluster]?, Error?) -> Void)) -> Void {
        var path = "/clusters?"
        
        if let date_minimum {
            path += "date_minimum=\(date_minimum)&"
        }
        if let date_maximum {
            path += "date_maximum=\(date_maximum)&"
        }
        
        network.req(path, method: .get, f: f)
    }
}
