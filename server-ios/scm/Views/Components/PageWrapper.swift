//
//  PageWrapper.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 08/10/24.
//


import SwiftUI

struct PageWrapper<Header: View, Content: View, Footer: View, Modifier: View>: View {
    @Environment(\.safeAreaInsets) private var safeAreaInsets
    
    @ViewBuilder let header: Header
    @ViewBuilder let content: Content
    @ViewBuilder let footer: Footer?
    @ViewBuilder let modifier: Modifier?
    
    private let title: String

    @State private var scrollState = (CGFloat(0.0), CGFloat(0.95))
    @State private var scroll: CGPoint = .zero
    
    private let VPW = UIScreen.main.bounds.size.width
    private let VPH = UIScreen.main.bounds.size.height
    
    init(
        @ViewBuilder header: () -> Header,
        @ViewBuilder content: () -> Content,
        @ViewBuilder modifier: () -> Modifier = { EmptyView() },
        title: String,
        @ViewBuilder footer: () -> Footer = { EmptyView() }
    ) {
        self.header = header()
        self.content = content()
        self.footer = nil
        self.modifier = modifier()
        self.title = title
    }
    init(
        @ViewBuilder header: () -> Header,
        @ViewBuilder content: () -> Content,
        @ViewBuilder footer: () -> Footer,
        @ViewBuilder modifier: () -> Modifier = { EmptyView() },
        title: String
    ) {
        self.header = header()
        self.content = content()
        self.footer = footer()
        self.modifier = modifier()
        self.title = title
    }
    
    var scrollPassed: Bool {
        if scroll.y <= -24 {
            return true
        } else {
            return false
        }
    }
    
    var body: some View {
        ZStack(alignment: .topLeading) {
            ScrollView {
                LazyVStack(spacing: 24) {
                    HStack {
                        TextHeadlineOne(title)
                            .foregroundStyle(.fontPrimary)
                            .lineLimit(1)
                        Spacer()
                        self.modifier
                    }
                    .zIndex(2)
                    
                    self.content
                }
                .padding(EdgeInsets(top: 84, leading: 24, bottom: footer != nil ? 96 : 24, trailing: 24))
                .background(GeometryReader { geometry in
                    Color.clear
                        .preference(key: ScrollOffsetPreferenceKey.self, value: geometry.frame(in: .named("scroll")).origin)
                })
                .onPreferenceChange(ScrollOffsetPreferenceKey.self) { value in
                    self.scroll = value
                }
            }
            .frame(alignment: .topLeading)
            .background(.backgroundTwo)
            .coordinateSpace(name: "scroll")
            
            ZStack {
                TextHeadlineFour(title)
                    .foregroundStyle(.fontPrimary)
                    .opacity(scrollState.0)
                    .scaleEffect(scrollState.1)
                HStack {
                    header
                }
            }
            .padding(EdgeInsets(top: safeAreaInsets.top + 12, leading: 24, bottom: 24, trailing: 24))
            .frame(width: self.VPW)
            .background(
                Rectangle()
                    .fill(.thinMaterial)
                    .opacity(scrollState.0)
            )
            .ignoresSafeArea()
            
            if let footer = self.footer {
                VStack(spacing: 0) {
                    Spacer()
                    ZStack {
                        VStack(alignment: .leading, spacing: 12) {
                            footer
                        }
                        .frame(height: 48)
                    }
                    .padding(EdgeInsets(top: 24, leading: 24, bottom: 24, trailing: 24))
                    .background(
                        ZStack(alignment: .topLeading) {
                            Rectangle()
                                .fill(.ultraThinMaterial)
                        }
                        .ignoresSafeArea()
                    )
                }
            }
        }
        .frame(width: VPW, alignment: .topLeading)
        .navigationBarBackButtonHidden(true)
        .onChange(of: self.scrollPassed) {
            if self.scrollPassed {
                withAnimation(.spring(duration: 0.25)) {
                    self.scrollState = (CGFloat(1), CGFloat(1))
                }
            } else {
                withAnimation(.spring(duration: 0.25)) {
                    self.scrollState = (CGFloat(0), CGFloat(0.95))
                }
            }
        }
    }
}
