//
//  ClusterForm.swift
//  scm
//
//  Created by Nuzulul Salsabila on 08/10/24.
//
import SwiftUI

struct ClusterForm: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(ClusterManager.self) private var clusterManager

    @State private var name: String = ""
    
    @State private var loading = true
    @State private var submitState = ButtonState.disabled
       
    let cluster: ViewCluster?

    init(_ cluster: ViewCluster? = nil) {
        self.cluster = cluster
    }

    var request: ClusterRequest? {
        if name.isEmpty {
            return nil
        }
        return ClusterRequest(name: name)
    }

    var body: some View {
        PageWrapper(
            header: {
                ButtonIcon("chevron.backward", type: .secondary) {
                    let _ = self.app.path.popLast()
                }
                Spacer()
            },
            content: {
                InputText("Cluster A", label: "Nama Cluster", text: $name)
            },
            footer: {
                ButtonRegular("SIMPAN CLUSTER", state: $submitState) {
                    self.submitState = .loading
                    if let request = request {
                        clusterManager.create(network, p: request) { cluster, err in
                            self.submitState = .idle
                            if let _ = cluster {
                                self.app.path.removeLast()
                            }
                        }
                    }
                }
            },
            title: "Tambah Cluster"
        )
        .onAppear {
            if let cluster = self.cluster {
                self.name = cluster.name
            }
        }
        .onChange(of: name) {
            self.submitState = name.isEmpty ? .disabled : .idle
        }
    }
}
