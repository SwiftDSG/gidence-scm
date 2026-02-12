//
//  ProcessorCard.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 05/03/25.
//

import SwiftUI

struct ProcessorCard: View {
    let processor: ViewProcessor
    let online: Bool
    let depth: CardDepth
    let open: (ViewCamera) -> Void
    
    init(_ processor: ViewProcessor, _ online: Bool, depth: CardDepth = .one, open: @escaping (ViewCamera) -> Void) {
        self.processor = processor
        self.online = online
        self.depth = depth
        self.open = open
    }
    
    var body: some View {
        VCard(
            header: {
                HStack(alignment: .center, spacing: 0) {
                    HStack(alignment: .center, spacing: 12) {
                        RoundedRectangle(cornerRadius: 12)
                            .fill(self.depth == .one ? .backgroundTwo : .backgroundOne)
                            .frame(width: 48, height: 48)
                            .overlay(
                                ZStack {
                                    Circle()
                                        .fill(self.online ? .success : .error)
                                        .frame(width: 24, height: 24)
                                        .opacity(0.2)
                                    Circle()
                                        .fill(self.online ? .success : .error)
                                        .frame(width: 10, height: 10)
                                }
                            )
                        VStack(alignment: .leading, spacing: 3) {
                            if depth == .one {
                                TextHeadlineFour(processor.name)
                            } else {
                                TextHeadlineFive(processor.name)
                            }
                            TextBodyFive(self.online ? "online" : "offline", color: .fontSecondary)
                        }
                    }
                    Spacer()
                    TextBodyFive("\(self.processor.camera.count) / 4 kamera", color: .fontSecondary)
                }
            },
            content: {
                HStack(alignment: .center, spacing: 6) {
                    Image(systemName: "exclamationmark.octagon.fill")
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .foregroundStyle(.error)
                        .frame(width: 24, height: 24)
                    TextHeadlineSix("\(self.processor.violation_count) pelanggaran tercatat", color: .fontSecondary)
                }
                .padding(12)
                .frame(maxWidth: .infinity, alignment: .topLeading)
                .background(
                    RoundedRectangle(cornerRadius: 12)
                        .fill(self.depth == .one ? .backgroundTwo : .backgroundOne)
                )
            },
            depth: self.depth
        )
    }
}
