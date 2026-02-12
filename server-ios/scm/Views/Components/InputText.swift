//
//  InputText.swift
//  staponic
//
//  Created by Kemal Dwi Heldy Muhammad on 08/06/24.
//

import SwiftUI

struct InputText: View {
    @Binding var state: InputState
    @Binding var text: String
    
    @FocusState private var isEditing: Bool
    
    let placeholder: String
    let label: String?
    let immediate: Bool
    let type: InputType
    
    init(_ placeholder: String, label: String? = nil, state: Binding<InputState> = Binding.constant(.idle), text: Binding<String>, immediate: Bool = false, type: InputType = .text) {
        self._state = state
        self._text = text
        self.placeholder = placeholder
        self.label = label
        self.immediate = immediate
        self.type = type
    }
    
    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            if type == .text {
                TextField(placeholder, text: $text)
                    .focused($isEditing)
                    .autocorrectionDisabled(true)
                    .textInputAutocapitalization(.never)
                    .textFieldStyle(SMSFieldStyle(isEditing: isEditing, label: label))
                    .opacity(state == .disabled ? 0.25 : 1)
                    .disabled(state == .disabled)
                    .onSubmit {
                        isEditing = false
                    }
                    .submitLabel(.done)
            } else if type == .secure {
                SecureField(placeholder, text: $text)
                    .focused($isEditing)
                    .textFieldStyle(SMSFieldStyle(isEditing: isEditing, label: label))
                    .opacity(state == .disabled ? 0.25 : 1)
                    .disabled(state == .disabled)
                    .onSubmit {
                        isEditing = false
                    }
                    .submitLabel(.done)
            }
            
            Rectangle()
                .fill(.clear)
                .overlay(
                    Image(systemName: "xmark")
                        .font(.system(size: 14))
                        .fontWeight(.bold)
                        .foregroundStyle(.fontPrimary)
                        .opacity(state == .disabled ? 0.25 : 1)
                )
                .frame(width: 48, height: 48)
                .opacity(text.count > 0 ? 1.0 : 0.0)
                .animation(.linear(duration: 0.25), value: text.count > 0)
                .onTapGesture {
                    text = ""
                    isEditing = true
                }
        }
        .frame(alignment: .bottomTrailing)
        .onAppear {
            if immediate {
                isEditing = true
            }
        }
    }
}
