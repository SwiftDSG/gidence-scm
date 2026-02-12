//
//  ClusterDetail.swift
//  scm
//
//  Created by Nuzulul Salsabila on 08/10/24.
//
import SwiftUI

struct ClusterDetail: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(ClusterManager.self) private var clusterManager
    @Environment(ProcessorManager.self) private var processorManager
    @Environment(EvidenceManager.self) private var evidenceManager
    @Environment(UserManager.self) private var userManager
    
    @State private var state = ButtonState.idle

    @State private var processors: [ViewProcessor] = []
    @State private var evidences: [ViewEvidence] = []
    @State private var users: [ViewUser] = []
    
    @State private var granularity = InputOption(key: "Hari ini", value: Date.now.currentTimeMillis())
    @State private var granularities = [
        InputOption(key: "Hari ini", value: Date.now.currentTimeMillis()),
        InputOption(key: "Minggu ini", value: Date.now.currentTimeMillis() - 86400000 * 7),
        InputOption(key: "Bulan ini", value: Date.now.currentTimeMillis() - 86400000 * 30),
    ]

    @State private var loading = true
    
    @State private var date_minimum: Int
    @State private var date_maximum: Int
    
    @State private var cluster: ViewCluster

    init(_ cluster: ViewCluster) {
        self.cluster = cluster
        
        let calendar = Calendar.current
        let now = Date()
        let x = calendar.startOfDay(for: now).currentTimeMillis()
        self.date_minimum = x
        self.date_maximum = x + 86400000
    }
    
    private let VPW = UIScreen.main.bounds.size.width
    
    var body: some View {
        PageWrapper(
            header: {
                ButtonIcon("chevron.backward", type: .secondary) {
                    let _ = self.app.path.popLast()
                }
                Spacer()
                
                InputSelect(self.$granularity, self.$granularities)
            },
            content: {
                VStack(spacing: 18) {
                    if !self.loading {
                        // MARK: - Main violations
                        if !self.evidences.isEmpty {
                            ScrollView(.horizontal) {
                                HStack(alignment: .top, spacing: 18) {
                                    ForEach(self.evidences, id: \.self) { evidence in
                                        ClusterEvidenceCard(evidence) {
                                            self.app.path.append(.EvidenceDetail(evidence))
                                        }
                                    }
                                }
                                .scrollTargetLayout()
                            }
                            .frame(width: self.VPW)
                            .contentMargins(.horizontal, 24) // Add padding
                            .scrollTargetBehavior(.viewAligned) // Align content behavior
                            .scrollIndicators(.hidden)
                        }
                        
                        // MARK: - Counter
                        HStack(spacing: 18) {
                            VStack(alignment: .leading, spacing: 18) {
                                ZStack {
                                    RoundedRectangle(cornerRadius: 12)
                                        .fill(.backgroundTwo)
                                        .frame(width: 48, height: 48)
                                    Image(systemName: "bell.fill")
                                        .resizable()
                                        .frame(width: 24, height: 24)
                                        .foregroundColor(.fontPrimary)
                                }
                                VStack(alignment: .leading, spacing: 3) {
                                    TextBodyThree("Peringatan", color: .fontSecondary)
                                    TextHeadlineTwo("\(self.cluster.notification_count)")
                                }
                            }
                            .padding(18)
                            .frame(width: (self.VPW - 66) / 2, alignment: .topLeading)
                            .background(
                                RoundedRectangle(cornerRadius: 18)
                                    .fill(.backgroundOne)
                            )
                            
                            VStack(alignment: .leading, spacing: 18) {
                                ZStack {
                                    RoundedRectangle(cornerRadius: 12)
                                        .fill(.backgroundTwo)
                                        .frame(width: 48, height: 48)
                                    Image(systemName: "exclamationmark.triangle.fill")
                                        .resizable()
                                        .frame(width: 24, height: 24)
                                        .foregroundColor(.warning)
                                }
                                VStack(alignment: .leading, spacing: 3) {
                                    TextBodyThree("Pelanggaran", color: .fontSecondary)
                                    TextHeadlineTwo("\(self.cluster.violation_count)")
                                }
                            }
                            .padding(18)
                            .frame(width: (VPW - 66) / 2, alignment: .topLeading)
                            .background(
                                RoundedRectangle(cornerRadius: 18)
                                    .fill(.backgroundOne)
                            )
                        }
                        
                        
                        
                        // MARK: - Processors
                        VCard(
                            header: {
                                VStack(alignment: .leading, spacing: 3) {
                                    TextHeadlineFour("Daftar processor")
                                    TextBodyFour("\(cluster.processor_count) processor terdaftar", color: .fontSecondary)
                                }
                                Spacer()
                            },
                            content: {
                                VStack(spacing: 12) {
                                    ForEach(self.processors, id: \.self) { processor in
                                        ProcessorCard(processor, true, depth: .two) { _ in
                                        }
                                    }
                                }
                            }
                        )
                        
                        // MARK: - User
                        VCard(
                            header: {
                                VStack(alignment: .leading, spacing: 3) {
                                    TextHeadlineFour("Daftar Petugas")
                                    TextBodyFour("\(self.users.count) petugas terdaftar", color: .fontSecondary)
                                }
                                Spacer()
                            },
                            content: {
                                VStack(spacing: 12) {
                                    ForEach(self.users, id: \.self) { user in
                                        UserCard(user, depth: .two)
                                    }
                                }
                            }
                        )
                    } else {
                        ProgressView()
                    }
                }
            },
            title: cluster.name
        )
        .onAppear {
            if self.network.authentication?.user.role == .officer {
                self.state = .disabled
            }
            self.load()
        }
        .onChange(of: self.granularity.value) {
            let date = if let t = self.granularity.value {
                Date(timeIntervalSince1970: Double(t / 1000))
            } else {
                Date.now
            }
            
            let calendar = Calendar.current
            let x = calendar.startOfDay(for: date).currentTimeMillis()
            self.date_minimum = x
            
            self.load()
        }
    }
    
    private func load() {
        self.loading = true
        self.clusterManager.get(self.network, cluster_id: self.cluster.id, date_minimum: self.date_minimum, date_maximum: self.date_maximum) { cluster, _ in
            if let cluster {
                self.cluster = cluster
            }
            self.evidenceManager.getMany(self.network, cluster_id: self.cluster.id, date_minimum: self.date_minimum, date_maximum: self.date_maximum) { evidences, _ in
                if let evidences {
                    self.evidences = evidences
                } else {
                    self.evidences = []
                }
                self.processorManager.getMany(self.network, cluster_id: self.cluster.id, date_minimum: self.date_minimum) { processors, _ in
                    if let processors {
                        self.processors = processors
                    } else {
                        self.processors = []
                    }
                    self.userManager.getMany(self.network, cluster_id: self.cluster.id, limit: 3) { users, _ in
                        self.loading = false
                        if let users {
                            self.users = users
                        }
                    }
                }
            }
        }
    }
}


