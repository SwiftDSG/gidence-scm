//
//  VCard.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 08/10/24.
//

import SwiftUI

struct HCard<Content: View>: View {
    @ViewBuilder let content: Content
    
    let depth: CardDepth

    private let VPW = UIScreen.main.bounds.size.width
    
    init(@ViewBuilder content: () -> Content, depth: CardDepth = .one) {
        self.content = content()
        self.depth = depth
    }
    
    var body: some View {
        if self.depth == .one {
            HStack(alignment: .center, spacing: 0) {
                content
            }
            .padding(18)
            .frame(width: VPW - 48, alignment: .topLeading)
            .background(
                RoundedRectangle(cornerRadius: 18)
                    .fill(Color.backgroundOne)
            )
        } else {
            HStack(alignment: .center, spacing: 0) {
                content
            }
            .padding(12)
            .frame(maxWidth: .infinity, alignment: .topLeading)
            .background(
                RoundedRectangle(cornerRadius: 12)
                    .fill(Color.backgroundTwo)
            )
        }
    }
}
