//
//  ButtonNavigation.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 27/10/24.
//

import SwiftUI

struct ButtonNavigation: View {
    @Binding private var stateNext: ButtonState
    @Binding private var statePrev: ButtonState

    let actionNext: (() -> Void)
    let actionPrev: (() -> Void)

    
    init(statePrev: Binding<ButtonState>, stateNext: Binding<ButtonState>, actionPrev: @escaping (() -> Void), actionNext: @escaping (() -> Void)) {
        self._statePrev = statePrev
        self._stateNext = stateNext
        self.actionPrev = actionPrev
        self.actionNext = actionNext
    }
    
    var body: some View {
        HStack(spacing: 0) {
            Button(action: actionPrev, label: {
                HStack {
                    Spacer()
                    if self.stateNext == .loading {
                        ProgressView()
                            .tint(.fontTertiary)
                    } else {
                        Image(systemName: "chevron.backward")
                            .foregroundStyle(.fontPrimary)
                            .font(.system(size: 14))
                            .fontWeight(.heavy)
                    }
                    Spacer()
                }
            })
            .frame(width: 48, height: 48)
            .buttonStyle(SMSButtonStyle(.secondary, icon: true, anchor: .trailing))
            .opacity(statePrev == .disabled ? 0.25 : 1)
            .disabled(statePrev == .disabled)
            
            Button(action: actionNext, label: {
                HStack {
                    Spacer()
                    if self.stateNext == .loading {
                        ProgressView()
                            .tint(.fontTertiary)
                    } else {
                        Image(systemName: "chevron.forward")
                            .foregroundStyle(.fontPrimary)
                            .font(.system(size: 14))
                            .fontWeight(.heavy)
                    }
                    Spacer()
                }
            })
            .frame(width: 48, height: 48)
            .buttonStyle(SMSButtonStyle(.secondary, icon: true, anchor: .leading))
            .opacity(stateNext == .disabled ? 0.25 : 1)
            .disabled(stateNext == .disabled)
        }
    }
}
