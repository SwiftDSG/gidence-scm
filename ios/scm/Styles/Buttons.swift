//
//  Buttons.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 07/10/24.
//

import SwiftUI

enum ButtonType {
    case primary
    case secondary
    case tertiary
    case quartenary
}

enum ButtonState {
    case idle
    case loading
    case disabled
}

struct SMSButtonStyle: ButtonStyle {
    let type: ButtonType
    let icon: Bool
    let radius: CGFloat
    let anchor: UnitPoint

    @State private var scale = CGFloat(1.0)
    @State private var opacity = CGFloat(0.0)
    @State private var pressing = false
    @State private var holding = false
    @State private var animating = false
    @State private var released = false
    
    init(_ type: ButtonType, icon: Bool = false, anchor: UnitPoint = .center, radius: CGFloat = 24) {
        self.type = type
        self.icon = icon
        self.anchor = anchor
        self.radius = radius
    }
    
    @ViewBuilder
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .frame(height: 48, alignment: .center)
            .background {
                ZStack {
                    switch type {
                        case .primary:
                            RoundedRectangle(cornerRadius: self.radius)
                                .fill(Color.one)
                                .strokeBorder(Color.one, lineWidth: 1)
                        case .secondary:
                            if self.anchor == .leading {
                                UnevenRoundedRectangle(cornerRadii: .init(
                                    topLeading: 0,
                                    bottomLeading: 0,
                                    bottomTrailing: self.radius,
                                    topTrailing: self.radius
                                ))
                                .stroke(.border, lineWidth: 1)
                            } else if self.anchor == .trailing {
                                UnevenRoundedRectangle(cornerRadii: .init(
                                    topLeading: self.radius,
                                    bottomLeading: self.radius,
                                    bottomTrailing: 0,
                                    topTrailing: 0
                                ))
                                .stroke(.border, lineWidth: 1)
                            } else {
                                RoundedRectangle(cornerRadius: self.radius)
                                    .fill(Color.backgroundOne)
                                    .strokeBorder(.border, lineWidth: 1)
                            }
                        case .tertiary:
                            RoundedRectangle(cornerRadius: self.radius)
                                .fill(Color.warning)
                                .strokeBorder(.border, lineWidth: 1)
                        case .quartenary:
                            RoundedRectangle(cornerRadius: self.radius)
                                .fill(Color.clear)
                    }
                    if self.anchor == .leading {
                        Rectangle()
                            .fill(.fontPrimary)
                            .opacity(opacity)
                            .clipShape(
                                .rect(
                                    topLeadingRadius: 0,
                                    bottomLeadingRadius: 0,
                                    bottomTrailingRadius: self.radius,
                                    topTrailingRadius: self.radius
                                )
                            )
                    } else if self.anchor == .trailing {
                        Rectangle()
                            .fill(.fontPrimary)
                            .opacity(opacity)
                            .clipShape(
                                .rect(
                                    topLeadingRadius: self.radius,
                                    bottomLeadingRadius: self.radius,
                                    bottomTrailingRadius: 0,
                                    topTrailingRadius: 0
                                )
                            )
                    } else {
                        RoundedRectangle(cornerRadius: self.radius)
                            .fill(.fontPrimary)
                            .opacity(opacity)
                    }
                }
            }
            .onChange(of: configuration.isPressed) {
                if configuration.isPressed {
                    animating = true
                    pressing = true
                    withAnimation(.spring(duration: 0.25)) {
                        if icon {
                            scale = CGFloat(0.9)
                        } else {
                            scale = CGFloat(0.95)
                        }
                        opacity = CGFloat(0.1)
                    } completion: {
                        finish()
                    }
                } else {
                    release()
                }
            }
            .scaleEffect(scale, anchor: anchor)
    }
    func finish() {
        animating = false
        if pressing {
            holding = true
        } else if released {
            reset()
        }
    }
    func release() {
        pressing = false
        if animating {
            released = true
        } else if holding {
            reset()
        }
    }
    func reset() {
        withAnimation(.spring(duration: 0.25)) {
            scale = 1.0
            opacity = 0.0
        }
    }
}
