//
//  Inputs.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 09/10/24.
//

import SwiftUI

enum InputState {
    case idle
    case disabled
}

enum InputType {
    case text
    case secure
}

struct SMSFieldStyle: TextFieldStyle {
    var isEditing: Bool
    let label: String?
    
    init(isEditing: Bool, label: String? = nil, clearable: Bool = false) {
        self.label = label
        self.isEditing = isEditing
    }
    
    func _body(configuration: TextField<_Label>) -> some View {
        VStack(spacing: 0) {
            if let label {
                HStack {
                    TextBodyFour(label, color: .fontSecondary)
                    Spacer()
                }
                .frame(height: 24)
            }
            HStack(alignment: .center) {
                configuration
                    .frame(height: 48)
                    .padding(EdgeInsets(top: 0, leading: 12, bottom: 0, trailing: 48))
                    .textFieldStyle(PlainTextFieldStyle())
                    .multilineTextAlignment(.leading)
                    .font(Font.system(size: 16))
                    .fontWeight(.regular)
                    .foregroundStyle(.fontPrimary)
            }
            .frame(height: 48)
            .background {
                ZStack {
                    RoundedRectangle(cornerRadius: 12, style: .continuous)
                        .strokeBorder(.border, lineWidth: 1)
                        .fill(.backgroundOne)
                    RoundedRectangle(cornerRadius: 12, style: .continuous)
                        .stroke(.one, lineWidth: isEditing ? 3 : 1)
                        .opacity(isEditing ? 0.25 : 0)
                    RoundedRectangle(cornerRadius: 12, style: .continuous)
                        .stroke(.border, lineWidth: 1)
                    RoundedRectangle(cornerRadius: 12, style: .continuous)
                        .strokeBorder(.one, lineWidth: 2)
                        .opacity(isEditing ? 1 : 0)
                }
            }
            .animation(.spring(duration: 0.25), value: isEditing)
        }
    }
}

struct SscmoggleStyle: ToggleStyle {
    func makeBody(configuration: Configuration) -> some View {
        HStack() {
            configuration.label
                .font(Font.system(size: 16))
                .fontWeight(.regular)
                .foregroundStyle(.fontPrimary)
            
            Spacer()
            
            RoundedRectangle(cornerRadius: 18)
                .fill(configuration.isOn ? .one : .backgroundOne)
                .strokeBorder(configuration.isOn ? .one : .border, lineWidth: 1)
                .frame(width: 48, height: 36)
                .overlay(
                    RoundedRectangle(cornerRadius: 12)
                        .fill(configuration.isOn ? .fontPrimary : .border)
                        .frame(width: configuration.isMixed ? 36 : 24, height: 24)
                        .position(x: configuration.isMixed ? 24 : 18, y: 18)
                        .offset(x: configuration.isOn ? 12 : 0)
                )
                .animation(.spring(duration: 0.25), value: configuration.isOn)
        }
        .onTapGesture {
            configuration.isOn.toggle()
        }
    }
}
