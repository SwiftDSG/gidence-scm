//
//  CameraCard.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 20/10/24.
//

import SwiftUI

struct CameraCard: View {
    let camera: ViewCamera
    let depth: CardDepth
    let open: () -> Void
    
    init(_ camera: ViewCamera, depth: CardDepth = .one, open: @escaping () -> Void) {
        self.camera = camera
        self.depth = depth
        self.open = open
    }
    
    var body: some View {
        HCard(
            content: {
                HStack(alignment: .center, spacing: 12) {
                    if self.depth == .one {
                        VStack(alignment: .leading, spacing: 3) {
                            TextHeadlineFour(camera.name)
                            TextBodyFour(self.camera.processor, color: .fontSecondary)
                        }
                    } else {
                        VStack(alignment: .leading, spacing: 3) {
                            TextHeadlineFive(camera.name)
                            TextBodyFour(self.camera.processor, color: .fontSecondary)
                        }
                    }
                }
                
                Spacer()
                
                if camera.violation_count > 0 || camera.notification_count > 0 {
                    HStack(spacing: 18) {
                        if camera.notification_count > 0 {
                            HStack(spacing: 6) {
                                TextHeadlineFive("\(camera.notification_count)", color: . fontPrimary)
                                Image(systemName: "bell.fill")
                                    .resizable()
                                    .frame(width: 14, height: 14)
                                    .foregroundColor(.fontPrimary)
                                
                            }
                        } else {
                            EmptyView()
                        }
                        
                        if camera.violation_count > 0 {
                            HStack(spacing: 6) {
                                TextHeadlineFive("\(camera.violation_count)", color: . fontPrimary)
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .resizable()
                                    .frame(width: 14, height: 14)
                                    .foregroundColor(.warning)
                                
                            }
                        } else {
                            EmptyView()
                        }
                    }
                }
            },
            depth: self.depth
        )
        .onTapGesture {
            self.open()
        }
    }
}
