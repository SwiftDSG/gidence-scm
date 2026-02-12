//
//  ClusterList.swift
//  scm
//
//  Created by Nuzulul Salsabila on 08/10/24.
//

import SwiftUI

struct ClusterList: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(ViewManager.self) private var viewManager
    @Environment(ClusterManager.self) private var clusterManager
    
    @State private var loading = true
    @State private var search = ""
    
    @State private var date_minimum: Int
    @State private var date_maximum: Int

    init() {
        let calendar = Calendar.current
        let now = Date()
        let x = calendar.startOfDay(for: now)
        self.date_minimum = Int(x.timeIntervalSince1970 * 1000)
        self.date_maximum = Int(x.timeIntervalSince1970 * 1000) + 86400000
    }
    
    private var clustersFilter: [ViewCluster] {
        if search .isEmpty {
            return viewManager.clusterList.clusters
        } else {
            return viewManager.clusterList.clusters.filter { cluster in
                cluster.name.lowercased().contains(search.lowercased())
            }
        }
    }


    var body: some View {
        PageWrapper(
            header: {
                ButtonIcon("person", type: .secondary) {
                    self.app.path.append(.Setting)
                }
                
                Spacer()
                
                if self.network.authentication?.user.role != .officer {
                    ButtonIcon("plus", type: .primary) {
                        self.app.path.append(.ClusterForm(nil))
                    }
                }
            },
            content: {
                VStack(spacing: 18) {
                    if self.loading {
                        ProgressView()
                    } else {
                        if self.viewManager.clusterList.clusters.isEmpty {
                            VStack(spacing: 24) {
                                Image("cluster.empty")
                                    .resizable()
                                    .aspectRatio(contentMode: .fit)
                                    .frame(maxWidth: .infinity)
                                
                                VStack(spacing: 3) {
                                    TextHeadlineFour("Data masih kosong")
                                    TextBodyFour("Tidak ada cluster yang ditemukan", color: .fontSecondary)
                                }
                            }
                            .padding(.top, 24)
                        } else {
                            InputText("Cari cluster...", text: $search)
                            
                            if self.clustersFilter.isEmpty {
                                VStack {
                                    TextBodyFour("Tidak ada cluster yang ditemukan", color: .fontSecondary)
                                        .frame(maxWidth: .infinity)
                                        .multilineTextAlignment(.center)
                                }
                                .frame(maxHeight: .infinity)
                                .padding(.top, 200)
                            } else {
                                ForEach(self.clustersFilter, id: \.self) { cluster in
                                    ClusterCard(cluster) {
                                        self.app.path.append(.ClusterDetail(cluster))
                                    }
                                }
                            }
                        }
                        Rectangle()
                            .fill(.clear)
                            .frame(height: 48)
                    }
                }
            },
            title: "Daftar cluster"
        )
        .onAppear {
            if self.viewManager.clusterList.clusters.isEmpty {
                self.loading = true
            } else {
                self.loading = false
            }
            
            self.clusterManager.getMany(self.network, date_minimum: self.date_minimum, date_maximum: self.date_maximum) { clusters, err in
                self.loading = false
                if let clusters {
                    self.viewManager.clusterList.clusters = clusters
                    print(clusters)
                }
            }
        }
    }
}
