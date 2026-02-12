//
//  Setting.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 31/10/24.
//

import SwiftUI

struct Setting: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(ViewManager.self) private var viewManager
    @Environment(NotificationManager.self) private var notificationManager

    @State private var toggleState = false
    @State private var toggleEnabled = InputState.idle
    @State private var initialized = false
    
    var body: some View {
        PageWrapper(
            header: {
                ButtonIcon("chevron.backward", type: .secondary) {
                    let _ = self.app.path.popLast()
                }
                Spacer()
            },
            content: {
                VStack(spacing: 18) {
                    if let user = self.network.authentication?.user {
                        HCard(
                            content: {
                                VStack(alignment: .leading, spacing: 3) {
                                    TextHeadlineFour(user.name)
                                    TextBodyFour(user.role.toString(), color: Color.fontSecondary)
                                }
                                Spacer()
                            }
                        )
                        
                        // MARK: Notification
                        HCard(
                            content: {
                                VStack(alignment: .leading, spacing: 3) {
                                    TextHeadlineFour("Notifikasi")
                                    TextBodyFour(self.toggleEnabled == .disabled ? "Aktifkan di pengaturan perangkat" : "Ingatkan apabila ada kejadian", color: Color.fontSecondary)
                                }
                                Spacer()
                                InputToggle(self.$toggleState, state: self.$toggleEnabled)
                                    .onTapGesture {
                                        if self.toggleEnabled == .disabled {
                                            self.notificationManager.requesting = true
                                        }
                                    }
                            }
                        )
                        
                        // MARK: Update profile
                        HCard(
                            content: {
                                VStack(alignment: .leading, spacing: 3) {
                                    TextHeadlineFour("Ubah Profil")
                                    TextBodyFour("Ubah nama profil atau kata sandi", color: Color.fontSecondary)
                                }
                                Spacer()
                                ButtonIcon("pencil", type: .secondary) {
                                    self.app.path.append(.SettingUser(user))
                                }
                            }
                        )
                        
                        // MARK: Logout
                        HCard(
                            content: {
                                VStack(alignment: .leading, spacing: 3) {
                                    TextHeadlineFour("Logout")
                                    TextBodyFour("Keluar akun", color: Color.fontSecondary)
                                }
                                Spacer()
                                ButtonIcon("rectangle.portrait.and.arrow.right", type: .secondary) {
                                    self.viewManager.clear()
                                    self.app.page = .EvidenceList
                                    self.network.authentication = nil
                                    let _ = self.app.path.popLast()
                                }
                            }
                        )
                    }
                }
            },
            title: "Pengaturan"
        )
        .onAppear {
            self.notificationManager.checkPermission() { permission in
                if permission == .enabled {
                    self.toggleState = true
                } else if permission == .unauthorized {
                    self.toggleEnabled = .disabled
                }
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
                    self.initialized = true
                }
            }
        }
        .onChange(of: self.toggleState) {
            if self.initialized {
                self.notificationManager.isPermitted() { permitted in
                    if self.toggleState && !permitted {
                        self.notificationManager.requesting = true
                    } else if !self.toggleState && permitted {
                        self.notificationManager.unsubscribe(self.network) { success, _ in
                            if success {
                                self.notificationManager.reject()
                            }
                        }
                    } else if self.toggleState && permitted {
                        self.notificationManager.subscribe(self.network) { success, _ in
                            if let success, success {
                                self.notificationManager.accept()
                            }
                        }
                    }
                }
            }
        }
        .onChange(of: self.notificationManager.requesting) {
            if !self.notificationManager.requesting {
                self.notificationManager.checkPermission() { permission in
                    if permission == .enabled {
                        self.toggleState = true
                    } else if permission == .unauthorized {
                        self.toggleState = false
                        self.toggleEnabled = .disabled
                    } else {
                        self.toggleState = false
                    }
                }
            }
        }
    }
}
