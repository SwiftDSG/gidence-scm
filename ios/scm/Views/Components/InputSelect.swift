//
//  InputSelect.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 30/10/24.
//

import SwiftUI

struct InputSelect<T>: View {
    @Binding var state: InputState
    @Binding var model: InputOption<T>
    @Binding var option: [InputOption<T>]
    
    @State private var editing = false
    @State private var options = [InputOption<T>]()
    
    let width = 120.0
    
    init(
        _ model: Binding<InputOption<T>>,
        _ options: Binding<[InputOption<T>]>,
        state: Binding<InputState> = Binding.constant(.idle)
    ) {
        self._state = state
        self._model = model
        self._option = options
    }
    
    var body: some View {
        ZStack(alignment: .topTrailing) {
            HStack(spacing: 0) {
                Spacer(minLength: 0)
                HStack(spacing: 0) {
                    TextHeadlineFive(self.model.key)
                    ZStack {
                        Image(systemName: "chevron.down")
                            .foregroundStyle(.fontPrimary)
                            .fontWeight(.bold)
                            .font(.system(size: 14))
                    }
                    .frame(width: 36, height: 36)
                    .rotationEffect(.degrees(self.editing ? 180 : 0))
                    .opacity(self.state == .disabled ? 0.25 : 1)
                    .animation(.spring(duration: 0.25), value: self.editing)
                }
                .padding(EdgeInsets(top: 0, leading: 12, bottom: 0, trailing: 0))
                .frame(height: 48)
                .background(
                    RoundedRectangle(cornerRadius: 18)
                        .fill(.backgroundOne)
                        .strokeBorder(.border, lineWidth: 1)
                )
            }
            .overlay(alignment: .topLeading) {
                ScrollView {
                    VStack(alignment: .leading, spacing: 0) {
                        ForEach(Array(self.options.enumerated()), id: \.offset) { index, option in
                            Button {
                                self.editing = false
                                self.model = option
                            } label: {
                                HStack {
                                    TextBodyFour(option.key, color: .fontSecondary)
                                        .lineLimit(1)
                                    Spacer()
                                }
                                .padding(EdgeInsets(top: 0, leading: 18, bottom: 0, trailing: 18))
                                .frame(maxWidth: .infinity, maxHeight: .infinity)
                            }
                            .frame(width: self.width, height: 36)
                        }
                    }
                }
                .frame(height: self.options.count > 4 ? 168 : CGFloat(self.options.count * 36))
                .background(.ultraThinMaterial)
                .scaleEffect(self.editing ? 1 : 1.1)
                .blur(radius: self.editing ? 0 : 12)
                .opacity(self.editing ? 1 : 0)
                .animation(.spring(duration: 0.25), value: self.editing)
                .clipShape(.rect(cornerRadius: 18))
                .offset(y: 36)
            }
        }
        .frame(width: self.width, alignment: .center)
        .onTapGesture {
            self.editing = true
        }
        .onAppear {
            self.options = self.option
        }
        .onChange(of: self.model.key, initial: true) {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.25) {
                if self.model.key.isEmpty {
                    self.options = self.option
                } else {
                    self.options = self.option.filter({ $0.key != self.model.key })
                }
            }
        }
        .onTapBackground(enabled: self.editing) {
            self.editing = false
        }
    }
}

