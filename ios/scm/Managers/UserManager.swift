//
//  UserManager.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 17/10/24.
//

import SwiftUI

@Observable class UserManager {
    func create(_ network: Network, p: UserRequest, f: @escaping ((ViewUser?, Error?) -> Void)) -> Void {
        do {
            let path = "/users"
            
            let data = try JSONEncoder().encode(p)
            
            network.req(path, method: .post, data: data) { (data, response, error) in
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
                            let decoded = try JSONDecoder().decode(ViewUser.self, from: data)
                            f(decoded, nil)
                        } catch let error {
                            print("Error: ", error)
                            f(nil, error)
                        }
                    }
                }
            }
        } catch let error {
            print("Error: ", error)
            f(nil, error)
        }
    }
    
    func login(_ network: Network, p: UserAuthenticationRequest, f: @escaping ((UserAuthentication?, Error?) -> Void)) -> Void {
        do {
            let path = "/users/login"
            
            let data = try JSONEncoder().encode(p)
            
            network.req(path, method: .post, data: data) { (data, response, error) in
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
                            let decoded = try JSONDecoder().decode(UserAuthentication.self, from: data)
                            f(decoded, nil)
                        } catch let error {
                            print("Error: ", error)
                            f(nil, error)
                        }
                    }
                }
            }
        } catch let error {
            print("Error: ", error)
            f(nil, error)
        }
    }
    
    func refresh(_ network: Network, p: UserRefreshRequest, f: @escaping ((UserAuthentication?, Error?) -> Void)) -> Void {
        do {
            let path = "/users/refresh"
            
            let data = try JSONEncoder().encode(p)
            
            network.req(path, method: .post, data: data) { (data, response, error) in
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
                            let decoded = try JSONDecoder().decode(UserAuthentication.self, from: data)
                            f(decoded, nil)
                        } catch let error {
                            print("Error: ", error)
                            f(nil, error)
                        }
                    }
                }
            }
        } catch let error {
            print("Error: ", error)
            f(nil, error)
        }
    }
    
    func update(_ network: Network, user_id: String, p: UserRequest, f: @escaping ((ViewUser?, Error?) -> Void)) -> Void {
        do {
            let path = "/users/\(user_id)"
            
            let data = try JSONEncoder().encode(p)
            
            network.req(path, method: .put, data: data) { (data, response, error) in
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
                            let decoded = try JSONDecoder().decode(ViewUser.self, from: data)
                            f(decoded, nil)
                        } catch let error {
                            print("Error: ", error)
                            f(nil, error)
                        }
                    }
                }
            }
        } catch let error {
            print("Error: ", error)
            f(nil, error)
        }
    }
    
    func get(_ network: Network, user_id: String, f: @escaping ((User?, Error?) -> Void)) -> Void {
        let path = "/users/\(user_id)"
        
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
                        let decoded = try JSONDecoder().decode(User.self, from: data)
                        f(decoded, nil)
                    } catch let error {
                        print("Error: ", error)
                        f(nil, error)
                    }
                }
            }
        }
    }
    
    func getMany(_ network: Network, text: String? = nil, cluster_id: String? = nil, cluster_eid: String? = nil, limit: Int? = nil, skip: Int? = nil, f: @escaping (([ViewUser]?, Error?) -> Void)) -> Void {
        var path = "/users?"
        
        if let text {
            path += "text=\(text)&"
        }
        if let cluster_id {
            path += "cluster_id=\(cluster_id)&"
        }
        if let cluster_eid {
            path += "cluster_eid=\(cluster_eid)&"
        }
        if let limit {
            path += "limit=\(limit)&"
        }
        if let skip {
            path += "skip=\(skip)&"
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
                        let decoded = try JSONDecoder().decode([ViewUser].self, from: data)
                        f(decoded, nil)
                    } catch let error {
                        print("Error: ", error)
                        f(nil, error)
                    }
                }
            }
        }
    }
    
    func delete(_ network: Network,  user_id: String, f: @escaping ((Bool?, Error?) -> Void)) -> Void {
        
        let path = "/users/\(user_id)"
        
        network.req(path, method: .delete) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response {
                print("Response: ", response)
            }
            f(true, nil)
        }
    }
}
