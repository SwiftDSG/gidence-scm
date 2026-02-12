//
//  InputTextSelect.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 30/10/24.
//

import SwiftUI

struct InputOption<T> {
    var key: String
    var value: T?
}

struct InputTextSelect<T>: View {
    @Binding var state: InputState
    @Binding var model: InputOption<T>
    @Binding var option: [InputOption<T>]
    
    @FocusState private var isEditing: Bool
    @State private var options = [InputOption<T>]()
    
    let placeholder: String
    let label: String?
    let immediate: Bool
    
    init(
        _ placeholder: String,
        label: String? = nil,
        state: Binding<InputState> = Binding.constant(.idle),
        model: Binding<InputOption<T>>,
        options: Binding<[InputOption<T>]>,
        immediate: Bool = false
    ) {
        self._state = state
        self._model = model
        self._option = options
        self.placeholder = placeholder
        self.label = label
        self.immediate = immediate
    }
    
    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            TextField(placeholder, text: $model.key)
                .focused($isEditing)
                .autocorrectionDisabled(true)
                .textInputAutocapitalization(.never)
                .textFieldStyle(SMSFieldStyle(isEditing: isEditing, label: label))
                .opacity(state == .disabled ? 0.25 : 1)
                .disabled(state == .disabled)
                .onSubmit {
                    isEditing = false
                }
                .overlay(alignment: .bottomTrailing) {
                    ZStack {
                        Image(systemName: "chevron.down")
                            .foregroundStyle(.fontPrimary)
                            .fontWeight(.bold)
                            .font(.system(size: 14))
                    }
                    .frame(width: 48, height: 48)
                    .rotationEffect(.degrees(isEditing ? 180 : 0))
                    .opacity(state == .disabled ? 0.25 : 1)
                    .animation(.spring(duration: 0.25), value: isEditing)
                    .onTapGesture {
                        self.isEditing.toggle()
                    }
                }
                .submitLabel(.done)
            
//            Rectangle()
//                .fill(.clear)
//                .overlay(
//                    Image(systemName: "xmark")
//                        .font(.system(size: 14))
//                        .fontWeight(.bold)
//                        .foregroundStyle(.fontPrimary)
//                )
//                .frame(width: 48, height: 48)
//                .opacity(model.key.count > 0 ? 1.0 : 0.0)
//                .animation(.linear(duration: 0.25), value: model.key.count > 0)
//                .onTapGesture {
//                    model = InputOption(key: "", value: nil)
//                    isEditing = true
//                }
        }
        .frame(alignment: .bottomTrailing)
        .overlay(alignment: .topLeading) {
            ScrollView {
                VStack(spacing: 0) {
                    ForEach(Array(options.enumerated()), id: \.offset) { index, option in
                        Button {
                            self.isEditing = false
                            self.model = option
                        } label: {
                            ZStack {
                                Rectangle()
                                    .fill(.one)
                                    .opacity(index % 2 == 0 ? 0.1 : 0)
                                HStack {
                                    TextBodyFour(option.key, color: .fontSecondary)
                                    Spacer()
                                }
                                .frame(height: 48)
                                .padding(EdgeInsets(top: 0, leading: 18, bottom: 0, trailing: 18))
                            }
                        }
                    }
                }
            }
            .frame(height: options.count > 4 ? 216 : CGFloat(options.count * 48))
            .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 12, style: .continuous))
            .scaleEffect(isEditing ? 1 : 1.1)
            .blur(radius: isEditing ? 0 : 12)
            .opacity(isEditing ? 1 : 0)
            .animation(.spring(duration: 0.25), value: isEditing)
            .clipShape(.rect(cornerRadius: 12))
            .offset(y: self.label != nil ? 72 : 48)
        }
        .onAppear {
            if immediate {
                isEditing = true
            }
            options = option
        }
        .onChange(of: model.key) {
            if model.key.isEmpty {
                self.options = self.option
            } else {
                self.options = self.option.filter({ $0.key.lowercased().contains(model.key.lowercased()) })
            }
        }
    }
}
