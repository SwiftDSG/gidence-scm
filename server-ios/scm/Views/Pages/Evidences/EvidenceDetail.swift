//
//  EvidenceDetail.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 10/11/24.
//

import SwiftUI

struct EvidenceDetail: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(EvidenceManager.self) private var evidenceManager
    
    // Image viewer state
    // The viewer zooms into each person's bounding box, with left/right buttons to cycle through persons.
    // Coordinate system: all bboxes are normalized (0.0–1.0), mapped to pixel space via currentImageSize.
    // The ZStack (image + bbox overlays) is sized in pixel space, then scaled/offset to focus on a person.
    @State private var currentImage: UIImage? = nil
    @State private var currentImageSize: CGSize = .zero   // Original image dimensions in pixels
    @State private var currentPersonIndex: Int = -1        // -1 = no person selected yet
    @State private var currentScaleFactor: CGFloat = 1     // Zoom level to fit person bbox in view
    @State private var currentOffset: CGPoint = .zero      // Pan offset to center person bbox in view
    
    @State private var sharing = false

    @State private var evidence: ViewEvidence
    
    private let VPW = UIScreen.main.bounds.size.width
    
    init(_ evidence: ViewEvidence) {
        self.evidence = evidence
    }
    
    private func formatTimestamp(_ timestamp: Int64) -> String {
        let date = Date(timeIntervalSince1970: Double(timestamp) / 1000) // Konversi ke detik
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "HH:mm - dd/MM/yyyy" // Format yang diinginkan
        dateFormatter.locale = Locale(identifier: "id_ID") // Bahasa Indonesia
        return dateFormatter.string(from: date)
    }
    
    var violator: Int {
        self.evidence.person.filter { $0.violation.count > 0 }.count
    }
    
    var path: String {
        "\(self.evidence.id).jpg"
    }
    
    // Sorted left-to-right by x position so navigation feels spatial
    var persons: [EvidencePerson] {
        self.evidence.person
            .sorted { $0.bbox[0] < $1.bbox[0] }
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
                VStack(alignment: .leading, spacing: 18) {
                    VCard(
                        header: {
                            HStack(spacing: 12) {
                                RoundedRectangle(cornerRadius: 12)
                                    .fill(.backgroundTwo)
                                    .frame(width: 48, height: 48)
                                    .overlay(
                                        Image(systemName: "video.fill")
                                            .foregroundStyle(.fontPrimary)
                                            .font(.system(size: 14))
                                            .fontWeight(.heavy)
                                    )
                                
                                VStack(alignment: .leading, spacing: 3) {
                                    TextBodyFour("Kamera", color: .fontSecondary)
                                    TextHeadlineFour(self.evidence.processor.name)
                                }
                            }
                        },
                        content: {
                            HStack(spacing: 0) {
                                VStack(alignment: .leading, spacing: 3) {
                                    TextBodyFour("Tanggal", color: .fontSecondary)
                                    TextHeadlineFive(self.formatTimestamp(self.evidence.timestamp))
                                }
                                
                                Spacer()
                                
                                VStack(alignment: .leading, spacing: 3) {
                                    TextBodyFour("Jumlah orang", color: .fontSecondary)
                                    TextHeadlineFive("\(self.violator) orang")
                                }
                                
                                Spacer()
                            }
                        }
                    )

                    Rectangle()
                        .fill(.backgroundOne)
                        .frame(width: VPW - 48, height: VPW - 48)
                        .overlay(
                            GeometryReader { geometry in
                                // Image + bbox overlays in pixel-space coordinates.
                                // The whole ZStack is sized to the original image dimensions,
                                // then offset and scaled to zoom into the selected person.
                                ZStack(alignment: .topLeading) {
                                    if let image = self.currentImage {
                                        Image(uiImage: image)
                                            .resizable()
                                            .aspectRatio(contentMode: .fit)

                                    } else {
                                        ProgressView()
                                    }

                                    if currentPersonIndex >= 0 && currentPersonIndex < persons.count {
                                        // All persons — active one highlighted yellow, others dimmed
                                        ForEach(persons.indices, id: \.self) { i in
                                            let p = persons[i]
                                            bboxRect(p.bbox, color: .white)
                                            
                                            // Body parts — red if the related PPE is missing/improper
                                            ForEach(p.part, id: \.self) { part in
                                                switch part.label {
                                                case .head:
                                                    bboxRect(part.bbox, color: p.violation.contains(where: { $0 == .missing_hardhat }) ? .error : .success)
                                                case .hand:
                                                    bboxRect(part.bbox, color: p.violation.contains(where: { $0 == .missing_gloves || $0 == .improperly_worn_gloves }) ? .error : .success)
                                                case .foot:
                                                    bboxRect(part.bbox, color: p.violation.contains(where: { $0 == .missing_shoes || $0 == .improperly_worn_shoes }) ? .error : .success)
                                                case .face:
                                                    bboxRect(part.bbox, color: p.violation.contains(where: { $0 == .missing_facemask || $0 == .improperly_worn_facemask }) ? .error : .success)
                                                case .ear:
                                                    bboxRect(part.bbox, color: p.violation.contains(where: { $0 == .missing_earmuffs || $0 == .improperly_worn_earmuffs }) ? .error : .success)
                                                }
                                            }

                                            // Equipment — red if improperly worn (missing won't appear here, it shows on the body part)
                                            ForEach(p.equipment, id: \.self) { eq in
                                                switch eq.label {
                                                case .hardhat:
                                                    bboxRect(eq.bbox, color: p.violation.contains(where: { $0 == .improperly_worn_hardhat }) ? .error : .success)
                                                case .gloves:
                                                    bboxRect(eq.bbox, color: p.violation.contains(where: { $0 == .improperly_worn_gloves }) ? .error : .success)
                                                case .shoes:
                                                    bboxRect(eq.bbox, color: p.violation.contains(where: { $0 == .improperly_worn_shoes }) ? .error : .success)
                                                case .facemask:
                                                    bboxRect(eq.bbox, color: p.violation.contains(where: { $0 == .improperly_worn_facemask }) ? .error : .success)
                                                case .earmuffs:
                                                    bboxRect(eq.bbox, color: p.violation.contains(where: { $0 == .improperly_worn_earmuffs }) ? .error : .success)
                                                default:
                                                    bboxRect(eq.bbox, color: .success)
                                                }
                                            }
                                        }
                                    } else {
                                        EmptyView()
                                    }
                                }
                                .frame(width: currentImageSize.width, height: currentImageSize.height, alignment: .topLeading)
                                .offset(x: currentOffset.x, y: currentOffset.y)
                                .scaleEffect(currentScaleFactor, anchor: .topLeading)
                            }
                        )
                        .clipShape(RoundedRectangle(cornerRadius: 18))
                    
                    HStack(spacing: 0) {
                        ButtonIcon("chevron.left", type: .secondary) {
                            if self.currentPersonIndex > 0 {
                                self.currentPersonIndex -= 1
                            } else {
                                self.currentPersonIndex = self.persons.count - 1
                            }
                        }
                        
                        Spacer()
                        
                        if self.currentPersonIndex >= 0 {
                            VStack(alignment: .center, spacing: 3) {
                                TextHeadlineFour("Orang \(self.currentPersonIndex + 1) / \(self.persons.count)")
                                TextBodyFour("\(self.persons[self.currentPersonIndex].violation.count) pelanggaran", color: .fontSecondary)
                            }
                        } else {
                            EmptyView()
                        }
                        
                        
                        Spacer()
                        
                        ButtonIcon("chevron.right", type: .secondary) {
                            if self.currentPersonIndex < self.persons.count - 1 {
                                self.currentPersonIndex += 1
                            } else {
                                self.currentPersonIndex = 0
                            }
                        }
                        
                    }
                    
                    VStack(alignment: .leading, spacing: 12) {
                        if self.currentPersonIndex >= 0 {
                            ForEach(self.persons[self.currentPersonIndex].violation, id: \.self) { violation in
                                violationCard(violation)
                            }
                        } else {
                            EmptyView()
                        }
                    }
                }
            },
            title: "Detil peringatan"
        )
        .onAppear() {
            if let cachedImage = ImageCache.shared.image(for: self.path) {
                self.currentImage = cachedImage
                self.currentImageSize = cachedImage.size
                self.currentPersonIndex = 0
            } else {
                self.network.load(self.path) { data, response, err in
                    if let data, let uiImage = UIImage(data: data) {
                        ImageCache.shared.setImage(uiImage, for: self.path)
                        self.currentImage = uiImage
                        self.currentImageSize = uiImage.size
                        self.currentPersonIndex = 0
                    } else if err != nil {
                        self.currentImage = nil
                    }
                }
            }
        }
        // Zoom + pan calculation when the selected person changes.
        //
        // How it works:
        //   1. Get the person's normalized bbox (0.0–1.0)
        //   2. Add margin so they're not cropped to the edge
        //   3. Calculate scale so the padded bbox fills the view
        //   4. Calculate offset so the bbox is centered in the view
        //
        // The scale/offset are applied to the ZStack which is in pixel-space,
        // so we convert between normalized coords and pixel coords as needed.
        // The dominant axis (wider or taller bbox) determines the scale factor.
        .onChange(of: self.currentPersonIndex) {
            if self.currentPersonIndex < 0 {
                return
            }

            let person = self.persons[self.currentPersonIndex]

            let viewSize = VPW - 48
            let margin = 0.12

            // Normalized bbox dimensions
            let w = CGFloat(person.bbox[2] - person.bbox[0])
            let h = CGFloat(person.bbox[3] - person.bbox[1])

            var pw = CGFloat(0)
            var ph = CGFloat(0)
            var px = CGFloat(0)  // Top-left x of the padded region (in pixel space)
            var py = CGFloat(0)  // Top-left y of the padded region (in pixel space)
            var scale = 1.0

            if w > h {
                // Wider than tall — scale to fit width, center vertically
                pw = w * (1 + margin * 2)
                scale = 1.0 / pw * (viewSize / min(currentImageSize.width, currentImageSize.height))

                px = (CGFloat(person.bbox[0]) - ((pw - w) / 2)) * currentImageSize.width
                py = (CGFloat(person.bbox[1]) + (h / 2) - (viewSize * 0.5 / currentImageSize.height / scale)) * currentImageSize.height
            } else if w < h {
                // Taller than wide — scale to fit height, center horizontally
                ph = h * (1 + margin * 2)
                scale = 1.0 / ph * (viewSize / min(currentImageSize.width, currentImageSize.height))

                px = (CGFloat(person.bbox[0]) + (w / 2) - (viewSize * 0.5 / currentImageSize.width / scale)) * currentImageSize.width
                py = (CGFloat(person.bbox[1]) - ((ph - h) / 2)) * currentImageSize.height
            } else {
                // Square bbox — scale equally, center both axes
                pw = w * (1 + margin * 2)
                ph = h * (1 + margin * 2)
                scale = 1.0 / ph * (viewSize / min(currentImageSize.width, currentImageSize.height))

                px = (CGFloat(person.bbox[0]) - ((pw - w) / 2)) * currentImageSize.width
                py = (CGFloat(person.bbox[1]) - ((ph - h) / 2)) * currentImageSize.height
            }

            withAnimation(.easeInOut(duration: 0.35)) {
                // Negative offset because we're panning the image left/up to center the person
                self.currentScaleFactor = scale
                self.currentOffset = CGPoint(
                    x: -px,
                    y: -py
                )
            }
        }
    }
    
    // Convert a normalized bbox [xmin, ymin, xmax, ymax] to a stroked rectangle
    // in pixel-space coordinates, matching the ZStack's coordinate system.
    private func bboxRect(_ bbox: [Float], color: Color) -> some View {
        let x = CGFloat(bbox[0]) * currentImageSize.width
        let y = CGFloat(bbox[1]) * currentImageSize.height
        let w = CGFloat(bbox[2] - bbox[0]) * currentImageSize.width
        let h = CGFloat(bbox[3] - bbox[1]) * currentImageSize.height

        return Rectangle()
            .stroke(color, lineWidth: 2)
            .frame(width: w, height: h)
            .position(x: x + w / 2, y: y + h / 2)
    }
    
    private func violationCard(_ violation: EvidencePersonViolation) -> some View {
        var icon: String {
            if violation == .improperly_worn_hardhat || violation == .missing_hardhat {
                return "hardhat.primary"
            }
            if violation == .missing_facemask || violation == .improperly_worn_facemask {
                return "facemask.primary"
            }
            if violation == .missing_gloves {
                return "gloves.primary"
            }
            if violation == .missing_safetyvest {
                return "safetyvest.primary"
            }
            return "violator.primary"
        }
        
        var title: String {
            if violation == .improperly_worn_hardhat {
                return "Penggunaan hardhat salah"
            }
            if violation == .missing_hardhat {
                return "Tidak memakai hardhat"
            }
            if violation == .improperly_worn_facemask {
                return "Penggunaan facemask salah"
            }
            if violation == .missing_facemask {
                return "Tidak memakai facemask"
            }
            if violation == .improperly_worn_gloves {
                return "Penggunaan gloves salah"
            }
            if violation == .missing_gloves {
                return "Tidak memakai gloves"
            }
            if violation == .missing_safetyvest {
                return "Tidak memakai safetyvest"
            }
            return "Pelanggaran"
        }
        
        var message: String {
            if violation == .improperly_worn_hardhat {
                return "Terdeteksi adanya kepala dan hardhat, namun letak hardhat tidak ada di dalam kotak kepala."
            }
            if violation == .missing_hardhat {
                return "Terdeteksi tangan yang tidak terlindungi oleh hard hat."
            }
            if violation == .improperly_worn_facemask {
                return "Terdeteksi adanya wajah dan masker, namun masker tidak menutupi wajah."
            }
            if violation == .missing_facemask {
                return "Terdeteksi tangan yang tidak terlindungi oleh masker."
            }
            if violation == .improperly_worn_gloves {
                return "Terdeteksi adanya tangan dan gloves, namun gloves tidak menutupi tangan."
            }
            if violation == .missing_gloves {
                return "Terdeteksi tangan yang tidak terlindungi oleh sarung tangan."
            }
            if violation == .missing_facemask {
                return "Terdeteksi tangan yang tidak terlindungi oleh safety vest."
            }
            return "Terdeteksi adanya pelanggaran."
        }
        
        return VCard(
            header: {
                HStack(alignment: .center, spacing: 12) {
                    Rectangle()
                        .fill(.error)
                        .frame(width: 36, height: 36)
                        .overlay(
                            Image(icon)
                                .resizable()
                                .aspectRatio(contentMode: .fit)
                                .frame(width: 24, height: 24)
                        )
                        .clipShape(RoundedRectangle(cornerRadius: 12))
                    TextHeadlineFive(title)
                }
            },
            content: {
                HCard(
                    content: {
                        TextBodyFour(message, color: .fontSecondary)
                    },
                    depth: .two
                )
            }
        )
    }
}
