//
//  TextBodyFive.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 05/06/24.
//

import SwiftUI

struct TextBodyFive: View {
    let text: String
    let color: Color
    
    init(_ t: String, color: Color = .fontPrimary) {
        self.text = t
        self.color = color
    }
    
    var body: some View {
        Text(text)
            .font(Font.system(size: 12))
            .fontWeight(.regular)
            .foregroundStyle(color)
    }
}

#Preview {
    TextBodyFive("Test")
}
