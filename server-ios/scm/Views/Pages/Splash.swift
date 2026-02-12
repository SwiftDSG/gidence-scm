//
//  Splash.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 15/11/24.
//

import SwiftUI

struct Splash: View {
    @State private var state = (5.0, 0.0, 0.75)
    
    private let VPW = UIScreen.main.bounds.size.width
    
    var body: some View {
        Rectangle()
            .fill(.backgroundTwo)
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .ignoresSafeArea()
            .overlay(
                Image("icon")
                    .resizable()
                    .blur(radius: self.state.0)
                    .scaleEffect(self.state.2)
                    .opacity(self.state.1)
                    .aspectRatio(contentMode: .fit)
                    .frame(width: self.VPW / 2)
            )
            .onAppear {
                DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
                    withAnimation(.spring(duration: 0.75)) {
                        self.state = (0.0, 1.0, 1.0)
                    }
                }
            }
    }
}

#Preview {
    Splash()
}
