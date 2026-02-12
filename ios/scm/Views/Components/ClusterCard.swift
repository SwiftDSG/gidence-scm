//
//  ClusterCard.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 18/10/24.
//

import SwiftUI

struct ClusterCard: View {
    let cluster: ViewCluster
    let open: () -> Void
    
    init(_ cluster: ViewCluster, open: @escaping () -> Void) {
        self.cluster = cluster
        self.open = open
    }
    
    var body: some View {
        VCard(
            header: {
                VStack(alignment: .leading, spacing: 3) {
                    TextHeadlineFour(cluster.name)
                    TextBodyFour("\(cluster.processor_count) kamera", color: .fontSecondary)
                }
                Spacer()
            },
            content: {
                if cluster.violation_count > 0 || cluster.notification_count > 0 {
                    HStack(spacing: 12) {
                        if cluster.violation_count > 0 {
                            HCard(
                                content: {
                                    HStack(spacing: 9) {
                                        Image(systemName: "bell.fill")
                                            .resizable()
                                            .aspectRatio(contentMode: .fit)
                                            .frame(width: 18, height: 18)
                                            .foregroundColor(.fontPrimary)
                                        
                                        TextHeadlineFive("\(cluster.violation_count) peringatan")
                                            .lineLimit(1)
                                    }
                                },
                                depth: .two
                            )
                        }
                        if cluster.notification_count > 0 {
                            HCard(
                                content: {
                                    HStack(spacing: 9) {
                                        Image(systemName: "exclamationmark.triangle.fill")
                                            .resizable()
                                            .aspectRatio(contentMode: .fit)
                                            .frame(width: 18, height: 18)
                                            .foregroundColor(.warning)
                                        
                                        TextHeadlineFive("\(cluster.notification_count) pelanggaran")
                                            .lineLimit(1)
                                    }
                                },
                                depth: .two
                            )
                        }
                    }
                } else {
                    EmptyView()
                }
            }
        )
        .onTapGesture {
            open()
        }
    }
}
