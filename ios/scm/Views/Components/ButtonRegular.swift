//
//  ButtonRegular.swift
//  staponic
//
//  Created by Kemal Dwi Heldy Muhammad on 05/06/24.
//

import SwiftUI



struct ButtonRegular: View {
    @Binding var state: ButtonState
    @State private var opacity = 1.0
    
    let label: String
    let type: ButtonType
    let action: (() -> Void)
    
    init(_ label: String, state: Binding<ButtonState> = Binding.constant(.idle), type: ButtonType = .primary, action: @escaping (() -> Void)) {
        _state = state
        opacity = state.wrappedValue == .disabled ? 0.25 : 1.0
        
        self.label = label
        self.type = type
        self.action = action
    }
    
    var body: some View {
        Button(action: action, label: {
            ZStack {
                TextCaption(label, color: self.type == .primary ? Color(hex: 0x303030) : .fontPrimary)
                    .opacity(state == .loading ? 0 : 1)
                    .animation(.linear(duration: 0.1), value: state == .loading)
                
                ProgressView()
                    .tint(.fontTertiary)
                    .opacity(state == .loading ? 1 : 0)
                    .animation(.linear(duration: 0.1), value: state == .loading)
            }
            .frame(maxWidth: .infinity)
        })
        .buttonStyle(SMSButtonStyle(type, radius: 12))
        .opacity(opacity)
        .disabled(state == .disabled)
        .onChange(of: state) {
            if state == .disabled {
                withAnimation(.spring(duration: 0.25)) {
                    opacity = 0.5
                }
            } else {
                withAnimation(.spring(duration: 0.25)) {
                    opacity = 1.0
                }
            }
        }
    }
}

