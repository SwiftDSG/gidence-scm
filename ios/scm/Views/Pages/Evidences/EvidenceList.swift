//
//  EvidenceList.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 08/10/24.
//

import SwiftUI

struct EvidenceList: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(ViewManager.self) private var viewManager
    @Environment(ClusterManager.self) private var clusterManager
    @Environment(EvidenceManager.self) private var evidenceManager
    @Environment(UserManager.self) private var userManager

    @State private var loading = false

    @State private var date_minimum: Int
    @State private var date_maximum: Int

    private let VPW = UIScreen.main.bounds.size.width

    func formatTimestamp(_ timestamp: Int64) -> String {
        let date = Date(timeIntervalSince1970: Double(timestamp) / 1000)
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "HH:mm"
        dateFormatter.locale = Locale(identifier: "id_ID")
        return dateFormatter.string(from: date)
    }

    init() {
        let calendar = Calendar.current
        let now = Date()
        let x = calendar.startOfDay(for: now)
        self.date_minimum = Int(x.timeIntervalSince1970 * 1000)
        self.date_maximum = Int(x.timeIntervalSince1970 * 1000) + 86400000
    }

    var body: some View {
        PageWrapper(
            header: {
                ButtonIcon("person", type: .secondary) {
                    self.app.path.append(.Setting)
                }

                Spacer()
            },
            content: {
                if self.loading {
                    ProgressView()
                } else {
                    if self.viewManager.evidenceList.evidences.isEmpty {
                        VStack(spacing: 24) {
                            Image("warning.empty")
                                .resizable()
                                .aspectRatio(contentMode: .fit)
                                .frame(width: VPW / 2)

                            VStack(spacing: 3) {
                                TextHeadlineFour("Belum ada peringatan")
                                TextBodyFour("Teruskan kinerja baiknya.",color: .fontSecondary)
                            }
                        }
                        .padding(.top, 24)
                    } else {
                        VStack(alignment: .leading, spacing: 18) {
                            ForEach(
                                Array(
                                    self.viewManager.evidenceList.evidences
                                        .enumerated()), id: \.offset
                            ) { _, evidence in
                                EvidenceCard(evidence) {
                                    self.app.path.append(.EvidenceDetail(evidence))
                                }
                            }
                        }
                    }
                }
                Rectangle()
                    .fill(.clear)
                    .frame(height: 48)
            },
            title: "Peringatan hari ini"
        )
        .onChange(of: self.network.authentication) {
            if self.network.authentication == nil {
                self.viewManager.evidenceList.evidences = []
            } else {
                self.load()
            }
        }
        .onAppear {
            self.load()
        }
    }

    private func load() {
        if self.viewManager.evidenceList.evidences.isEmpty {
            self.loading = true
        } else {
            self.loading = false
        }

        if self.network.authentication != nil {
            self.evidenceManager.getMany(
                self.network, date_minimum: self.date_minimum,
                date_maximum: self.date_maximum
            ) { evidences, _ in
                self.loading = false

                if let evidences {
                    self.viewManager.evidenceList.evidences = evidences
                }
            }
        }
    }
}
