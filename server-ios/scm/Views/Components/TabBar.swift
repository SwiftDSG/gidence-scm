//
//  TabBar.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 30/10/24.
//

import SwiftUI

struct TabBar: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network

    let VPW = UIScreen.main.bounds.size.width
    let VPH = UIScreen.main.bounds.size.height
    
    var pages: [(PageKind, String, String)] {
        if self.network.authentication?.user.role == .super_admin || self.network.authentication?.user.role == .manager {
            return [
                (.EvidenceList, "bell", "Peringatan"),
                (.ClusterList, "rectangle.3.group", "Cluster"),
                (.UserList, "person.3.sequence", "Pekerja")
            ]
        }
        return [
            (.EvidenceList, "bell", "Peringatan"),
            (.ClusterList, "rectangle.3.group", "Cluster")
        ]
    }
    
    var body: some View {
        HStack(alignment: .center, spacing: 0) {
            ForEach(pages, id: \.self.0) { page in
                VStack(alignment: .center, spacing: 6) {
                    Image(systemName: page.1 + (app.page == page.0 ? ".fill" : ""))
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .frame(height: 18)
                        .scaleEffect(page.0 == app.page ? 1 : 0.9)
                        .foregroundStyle(page.0 == app.page ? .one : Color(hex: 0xC3C3C3, alpha: 0.75))
                        .animation(.spring(duration: 0.25), value: page.0 == app.page)
                    
                    TextHeadlineSix(page.2, color: page.0 == app.page ? Color(hex: 0xFFFFFF, alpha: 1.0) : Color(hex: 0xC3C3C3, alpha: 0.75))
                        .scaleEffect(page.0 == app.page ? 1 : 0.9)
                        .animation(.spring(duration: 0.25), value: page.0 == app.page)
                }
                .frame(width: (VPW - 66) / CGFloat(self.pages.count), height: 72)
                .onTapGesture {
                    app.page = page.0
                }
            }
        }
        .padding(.horizontal, 18)
        .frame(width: VPW - 48, height: 72)
        .background(
            ZStack {
                RoundedRectangle(cornerRadius: 36)
                    .fill(.fontTertiary)
                RoundedRectangle(cornerRadius: 36)
                    .fill(.ultraThinMaterial)
            }
        )
        .offset(y: app.path.count == 0 ? 0 : 72 + VPH * 0.25)
        .animation(.spring(duration: 0.5), value: app.path.count == 0)
    }
}
