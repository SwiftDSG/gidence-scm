//
//  User.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 28/10/24.
//

struct User: Hashable, Codable, Identifiable {
    var id: String
    var number: String
    var password: String
    var name: String
    var role: UserRole
    var cluster_id: [String]
}

enum UserRole: String, Codable {
    case manager
    case officer
    case super_admin
    
    func toString() -> String {
        switch self {
            case .manager:
                return "Manager"
            case .officer:
                return "Petugas"
            case .super_admin:
                return "Super admin"
        }
    }
}

struct UserRequest: Hashable, Codable {
    var number: String
    var password: String
    var name: String
    var role: UserRole
    var cluster_id: [String]
}

struct UserAuthentication: Hashable, Codable {
    var atk: String
    var rtk: String
    var user: ViewUser
}

struct UserAuthenticationRequest: Hashable, Codable {
    var number: String
    var password: String
}

struct UserRefreshRequest: Hashable, Codable {
    var rtk: String
    
    init(_ rtk: String) {
        self.rtk = rtk
    }
}

struct ViewUser: Hashable, Codable, Identifiable {
    var id: String
    var number: String
    var name: String
    var role: UserRole
    var cluster: [ClusterRef]
}
