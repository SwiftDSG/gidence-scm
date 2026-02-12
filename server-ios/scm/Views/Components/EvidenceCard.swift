//
//  EvidenceCard.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 08/11/24.
//

import SwiftUI
import Photos



struct EvidenceCard: View {
    @Environment(Network.self) private var network
    @Environment(EvidenceManager.self) private var evidenceManager
    @Environment(ProcessorManager.self) private var processorManager

    @State private var sharing = false
    
    private let VPW = UIScreen.main.bounds.size.width
    
    let evidence: ViewEvidence
    let action: () -> Void
        
    init(_ evidence: ViewEvidence, action: @escaping () -> Void) {
        self.evidence = evidence
        self.action = action
    }
    
    func formatTimestamp(_ timestamp: Int64) -> String {
        let date = Date(timeIntervalSince1970: Double(timestamp) / 1000)
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "HH:mm"
        dateFormatter.locale = Locale(identifier: "id_ID")
        return dateFormatter.string(from: date)
    }
    
    var violator: Int {
        self.evidence.person.filter { $0.violation.count > 0 }.count
    }
    
    var body: some View {
        VCard(
            header: {
                HStack(spacing: 0) {
                    VStack(alignment: .leading, spacing: 3) {
                        TextHeadlineFour(evidence.camera.name)
                        TextBodyFour(evidence.cluster.name, color: .fontSecondary)
                    }
                    
                    Spacer()
                    HStack(spacing: 6) {
                        TextCaption(
                            "\(formatTimestamp(evidence.timestamp))",
                            color: .fontSecondary
                        )
                        .padding(
                            EdgeInsets(
                                top: 0, leading: 8, bottom: 0,
                                trailing: 6)
                        )
                        .frame(height: 24)
                        .background(
                            RoundedRectangle(cornerRadius: 12)
                                .fill(.clear)
                                .strokeBorder(
                                    .border, lineWidth: 1)
                        )
                    }
                }
            }, content: {
                HCard(
                    content: {
                        HStack(spacing: 6) {
                            Image(systemName: "person.badge.shield.exclamationmark.fill")
                                .foregroundStyle(.error)
                                .font(.system(size: 14))
                                .fontWeight(.heavy)
                            
                            TextHeadlineFive(
                                "\(self.violator) orang melanggar"
                            )
                            .lineLimit(1)
                        }
                    },
                    depth: .two
                )
            }
        )
        .onTapGesture {
            self.action()
        }
    }
}
