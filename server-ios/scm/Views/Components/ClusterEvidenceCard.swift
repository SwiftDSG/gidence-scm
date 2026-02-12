import SwiftUI

struct ClusterEvidenceCard: View {
    let evidence: ViewEvidence
    let open: () -> Void
    
    init(_ evidence: ViewEvidence, open: @escaping () -> Void) {
        self.evidence = evidence
        self.open = open
    }
    
    private let VPW = UIScreen.main.bounds.size.width
    
    private func formatTimestamp(_ timestamp: Int64) -> String {
        let date = Date(timeIntervalSince1970: Double(timestamp) / 1000) // Konversi ke detik
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "HH:mm - dd MMMM yyyy" // Format yang diinginkan
        dateFormatter.locale = Locale(identifier: "id_ID") // Bahasa Indonesia
        return dateFormatter.string(from: date)
    }
    
    var violation: Int {
        return self.evidence.person.reduce(Int(), { a, b in
            return a + b.violation.count
        })
    }
    
    var path: String {
        "\(self.evidence.id).jpg"
    }
    
    var body: some View {
        ZStack(alignment: .leading) {
            Rectangle()
                .frame(width: VPW - 72, height: (VPW - 72) * 2 / 3)
                .overlay(
                    ZStack {
                        SCMImage(self.path)
                        VStack(spacing: 0) {
                            // Clickable shape, to prevent clicking through SCMImage
                            Color.clear
                                .contentShape(Rectangle())
                                .frame(width: VPW - 72, height: (VPW - 72) * 1 / 3)
                            VariableBlurView(maxBlurRadius: 2, direction: .blurredBottomClearTop)
                        }
                        .frame(width: VPW - 72, height: (VPW - 72) * 2 / 3)
                    }
                )
                .clipShape(RoundedRectangle(cornerRadius: 24))
            
            
            VStack(alignment: .leading) {
                Spacer()
                HStack(alignment: .bottom, spacing: 6) {
                    VStack(alignment: .leading, spacing: 3) {
                        TextHeadlineFour("\(self.evidence.camera.name)", color: .backgroundOne)
                        TextBodyFour("\(self.formatTimestamp(evidence.timestamp))", color: .backgroundTwo)
                    }
                    Spacer()
                    HStack(spacing: 6) {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundStyle(.fontPrimary)
                            .font(.system(size: 14))
                            .fontWeight(.heavy)
                        
                        TextHeadlineSix("\(self.violation)", color: .fontPrimary)
                    }
                    .padding(.horizontal, 6)
                    .frame(height: 24)
                    .background(
                        RoundedRectangle(cornerRadius: 12)
                            .fill(.warning)
                    )
                }
                .padding(18)
            }
        }
        .frame(width: self.VPW - 72)
        .onTapGesture {
            open()
        }
    }
}
