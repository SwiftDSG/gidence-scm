//
//  ViewManager.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 03/11/24.
//

import SwiftUI

@Observable class ViewManager {
    var evidenceList: EvidenceListData
    var clusterList: ClusterListData
    var userList: UserListData

    init() {
        self.evidenceList = EvidenceListData ()
        self.clusterList = ClusterListData()
        self.userList = UserListData()
    }
    
    func clear() {
        self.evidenceList = EvidenceListData ()
        self.clusterList = ClusterListData()
        self.userList = UserListData()
    }
}

struct EvidenceListData {
    var evidences: [ViewEvidence] = []
}
struct ClusterListData {
    var clusters: [ViewCluster] = []
}
struct UserListData {
    var users: [ViewUser] = []
}
