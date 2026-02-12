//
//  ButtonIcon.swift
//  staponic
//
//  Created by Kemal Dwi Heldy Muhammad on 05/06/24.
//

import SwiftUI

struct ButtonIcon: View {
    @Binding var state: ButtonState
    
    let icon: String
    let type: ButtonType
    let action: (() -> Void)
    
    @Environment(\.colorScheme) var colorScheme // Detect dark/light mode
    
    init(_ icon: String, state: Binding<ButtonState> = Binding.constant(.idle), type: ButtonType = .primary, action: @escaping (() -> Void)) {
        _state = state
        self.icon = icon
        self.type = type
        self.action = action
    }
    
    var body: some View {
        Button(action: action, label: {
            HStack {
                Spacer()
                if self.state == .loading {
                    ProgressView()
                        .tint(.fontTertiary)
                } else {
                    Image(systemName: icon)
                        .foregroundStyle(self.type == .primary ? Color(hex: 0x303030) : .fontPrimary)
                        .font(.system(size: 14))
                        .fontWeight(.heavy)
                }
                Spacer()
            }
        })
        .frame(width: 48, height: 48)
        .buttonStyle(SMSButtonStyle(type, icon: true, radius: 18))
        .opacity(state == .disabled ? 0.25 : 1)
        .disabled(state == .disabled)
    }
}
