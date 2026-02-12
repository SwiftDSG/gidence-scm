//
//  SettingUser.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 01/11/24.
//

import SwiftUI

struct SettingUser: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(UserManager.self) private var userManager
    
    @State private var name = ""
    @State private var oldPassword = ""
    @State private var newPassword = ""
    
    @State private var submitState = ButtonState.disabled
    
    let user: ViewUser
    
    init(_ user: ViewUser) {
        self.user = user
    }
    
    var request: UserRequest? {
        if self.name.isEmpty {
            return nil
        }
        
        return UserRequest(
            number: self.user.number,
            password: self.newPassword,
            name: self.name,
            role: self.user.role,
            cluster_id: []
        )
    }

    var body: some View {
        PageWrapper(
            header: {
                ButtonIcon("chevron.backward", type: .secondary) {
                    let _ = app.path.popLast()
                }
                Spacer()
            },
            content: {
                InputText("Masukkan nama petugas", label: "Nama Petugas", text: $name)
                InputText("Masukkan password lama", label: "Password Lama", text: $oldPassword, type: .secure)
                InputText("Masukkan password baru", label: "Password Baru", text: $newPassword, type: .secure)
            },
            footer: {
                ButtonRegular("SIMPAN", state: $submitState) {
                    if let request {
                        self.submitState = .loading
                        self.userManager.update(self.network, user_id: user.id, p: request) { user, err in
                            let _ = app.path.popLast()
                            self.submitState = .idle
                            if let user {
                                self.network.authentication?.user = user
                            }
                        }
                    }
                }   
            },
            title: "Ubah Profil"
        )
        .onChange(of: self.request) {
            if self.request != nil {
                self.submitState = .idle
            } else {
                self.submitState = .disabled
            }
        }
        .onAppear{
            self.name = self.user.name
        }
        
    }
}
