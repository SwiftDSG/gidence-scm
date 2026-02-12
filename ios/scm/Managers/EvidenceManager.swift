//
//  EvidenceManager.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 23/10/24.
//

import SwiftUI

@Observable class EvidenceManager {
    func getMany(_ network: Network, cluster_id: String? = nil, processor_id: String? = nil, camera_id: String? = nil, date_minimum: Int? = nil, date_maximum: Int? = nil, f: @escaping (([ViewEvidence]?, Error?) -> Void)) -> Void {
        var path = "/evidences?"
        
        if let cluster_id {
            path += "cluster_id=\(cluster_id)&"
        }
        if let processor_id {
            path += "processor_id=\(processor_id)&"
        }
        if let camera_id {
            path += "camera_id=\(camera_id)&"
        }
        if let date_minimum {
            path += "date_minimum=\(date_minimum)&"
        }
        if let date_maximum {
            path += "date_maximum=\(date_maximum)&"
        }
        
        network.req(path, method: .get) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response {
                print("Response: ", response)
            }
            if let data {
                DispatchQueue.main.async {
                    do {
                        let decoded = try JSONDecoder().decode([ViewEvidence].self, from: data)
                        f(decoded, nil)
                    } catch let error {
                        print("Error: ", error)
                        f(nil, error)
                    }
                }
            }
        }
    }
    func get(_ network: Network, _ id: String, f: @escaping ((ViewEvidence?, Error?) -> Void)) -> Void {
        var path = "/evidences/\(id)"
        
        network.req(path, method: .get) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response {
                print("Response: ", response)
            }
            if let data {
                DispatchQueue.main.async {
                    do {
                        let decoded = try JSONDecoder().decode(ViewEvidence.self, from: data)
                        f(decoded, nil)
                    } catch let error {
                        print("Error: ", error)
                        f(nil, error)
                    }
                }
            }
        }
    }
}
