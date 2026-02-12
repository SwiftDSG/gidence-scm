//
//  InputToggle.swift
//  staponic
//
//  Created by Kemal Dwi Heldy Muhammad on 08/06/24.
//

import SwiftUI

struct InputToggle: View {
    @Binding var toggled: Bool
    @Binding var state: InputState
    
    init(_ toggled: Binding<Bool> = Binding.constant(false), state: Binding<InputState> = Binding.constant(.idle)) {
        self._toggled = toggled
        self._state = state
    }
    
    var body: some View {
        RoundedRectangle(cornerRadius: 18)
            .fill(self.toggled ? .one : .backgroundOne)
            .strokeBorder(self.toggled ? .one : .border, lineWidth: 1)
            .frame(width: 48, height: 36)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .fill(self.toggled ? .fontTertiary : .border)
                    .frame(width: 24, height: 24)
                    .position(x: 18, y: 18)
                    .offset(x: self.toggled ? 12 : 0)
            )
            .opacity(self.state == .disabled ? 0.5 : 1)
            .animation(.spring(duration: 0.25), value: self.toggled)
            .onTapGesture {
                if self.state != .disabled {
                    self.toggled.toggle()
                }
            }
    }
}
