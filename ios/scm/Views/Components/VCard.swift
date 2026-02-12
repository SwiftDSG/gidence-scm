//
//  VCard.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 08/10/24.
//

import SwiftUI

struct VCard<Header: View, Content: View>: View {
    @ViewBuilder let header: Header
    @ViewBuilder let content: Content
    
    let depth: CardDepth

    private let VPW = UIScreen.main.bounds.size.width
    
    init(@ViewBuilder header: () -> Header, @ViewBuilder content: () -> Content, depth: CardDepth = .one) {
        self.header = header()
        self.content = content()
        self.depth = depth
    }
    
    var body: some View {
        if self.depth == .one {
            VStack(alignment: .leading, spacing: 18) {
                HStack(alignment: .center, spacing: 0) {
                    header
                }
                content
            }
            .padding(18)
            .frame(width: VPW - 48, alignment: .topLeading)
            .background(
                RoundedRectangle(cornerRadius: 18)
                    .fill(Color.backgroundOne)
            )
            .clipped()
        } else {
            VStack(spacing: 18) {
                HStack(alignment: .center, spacing: 0) {
                    header
                }
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
