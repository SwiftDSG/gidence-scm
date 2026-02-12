//
//  SCMImage.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 20/10/24.
//

import SwiftUI

struct SCMImage: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    
    @State private var image: UIImage? = nil
    @State private var state = SMSImageState.loading
    @State private var fullscreen = false

    let name: String
    
    init(_ name: String) {
        self.name = name
    }
    
    var body: some View {
        ZStack {
            if let image {
                Image(uiImage: image)
                    .resizable()
                    .aspectRatio(contentMode: .fill)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .onTapGesture {
                        self.app.image = image
                    }
            } else if state == .loading {
                Color.clear
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .overlay(
                        ProgressView()
                    )
                    .onAppear {
                        self.loadImage()
                    }
            } else {
                Color.clear
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .overlay(
                        Image(systemName: "exclamationmark.triangle")
                            .resizable()
                    )
            }
        }
    }
    
    private func loadImage() {
        if let cachedImage = ImageCache.shared.image(for: name) {
            self.image = cachedImage
            self.state = .loaded
        } else {
            self.network.load(name) { data, response, err in
                if let data, let uiImage = UIImage(data: data) {
                    self.state = .loaded
                    self.image = uiImage
                    ImageCache.shared.setImage(uiImage, for: name)
                    
                    if let response {
                        let cachedResponse = CachedURLResponse(response: response, data: data)
                        URLCache.shared.storeCachedResponse(cachedResponse, for: .init(url: response.url!))
                    }
                } else if err != nil {
                    self.state = .error
                }
            }
        }
    }
    
    private enum SMSImageState {
        case loading
        case loaded
        case error
    }
}

struct SCMImageFullScreen: View {
    @State private var buttonState = ButtonState.idle
    
    @State private var scale = 1.0
    @State private var scaleLast = 1.0

    @State private var offset: CGPoint = .zero
    @State private var offsetLast: CGSize = .zero
    
    @State private var opacity = 0.0
    
    let image: UIImage
    let close: () -> Void
    
    init(_ image: UIImage, close: @escaping () -> Void) {
        self.image = image
        self.close = close
    }

    var body: some View {
        ZStack {
            Rectangle()
                .fill(.ultraThinMaterial)
                .ignoresSafeArea()
                .onTapGesture {
                    withAnimation(.linear(duration: 0.25)) {
                        self.opacity = 0.0
                    } completion: {
                        close()
                    }
                }
            
            GeometryReader { proxy in
                VStack {
                    Spacer()
                    Image(uiImage: self.image)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .scaleEffect(scale)
                        .offset(x: offset.x, y: offset.y)
                        .gesture(makeDragGesture(proxy.size))
                        .gesture(makeMagnificationGesture(proxy.size))
                        .onTapGesture(count: 2) {
                            if self.scale == 1 {
                                withAnimation {
                                    self.scale = 3
                                    self.offset = .zero
                                }
                            } else {
                                self.scaleLast = 1
                                withAnimation {
                                    self.scale = 1
                                    self.offset = .zero
                                }
                            }
                        }
                    Spacer()
                }
                .edgesIgnoringSafeArea(.all)
            }
            
            VStack {
                HStack {
                    ButtonIcon("xmark", state: $buttonState, type: .secondary) {
                        withAnimation(.linear(duration: 0.25)) {
                            self.opacity = 0.0
                        } completion: {
                            close()
                        }
                    }
                    Spacer()
                }
                Spacer()
            }
            .padding(EdgeInsets(top: 12, leading: 24, bottom: 0, trailing: 24))
            .frame(alignment: .topLeading)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .opacity(self.opacity)
        .onAppear {
            withAnimation(.linear(duration: 0.25)) {
                self.opacity = 1.0
            }
        }
    }
    
    private func makeMagnificationGesture(_ size: CGSize) -> some Gesture {
        MagnificationGesture()
            .onChanged { value in
                let delta = value / self.scaleLast
                self.scaleLast = value
                
                if abs(1 - delta) > 0.01 {
                    self.scale *= delta
                }
            }
            .onEnded { _ in
                self.scaleLast = 1
                if self.scale < 1 {
                    withAnimation {
                        self.scale = 1
                    }
                }
                adjustMaxOffset(size)
            }
    }
    
    private func makeDragGesture(_ size: CGSize) -> some Gesture {
        DragGesture()
            .onChanged { value in
                let diff = CGPoint(
                    x: value.translation.width - self.offsetLast.width,
                    y: value.translation.height - self.offsetLast.height
                )
                self.offset = CGPoint(
                    x: self.offset.x + diff.x,
                    y: self.offset.y + diff.y
                )
                self.offsetLast = value.translation
            }
            .onEnded { _ in
                adjustMaxOffset(size)
            }
    }
    
    private func adjustMaxOffset(_ size: CGSize) {
        let maxOffsetX = (size.width * (self.scale - 1)) / 2
        let maxOffsetY = (size.height * (self.scale - 1)) / 2
        
        var newOffsetX = self.offset.x
        var newOffsetY = self.offset.y
        
        if abs(newOffsetX) > maxOffsetX {
            newOffsetX = maxOffsetX * (abs(newOffsetX) / newOffsetX)
        }
        if abs(newOffsetY) > maxOffsetY {
            newOffsetY = maxOffsetY * (abs(newOffsetY) / newOffsetY)
        }
        
        let newOffset = CGPoint(x: newOffsetX, y: newOffsetY)
        if newOffset != self.offset {
            withAnimation {
                self.offset = newOffset
            }
        }
        self.offsetLast = .zero
    }
}
