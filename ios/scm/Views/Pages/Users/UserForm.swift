//
//  UserForm.swift
//  scm
//
//  Created by Kathlyne Sarah on 09/10/24.
//

import SwiftUI

struct UserForm: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(UserManager.self) private var userManager
    
    @State private var number = ""
    @State private var name = ""
    @State private var role = InputOption<UserRole>(key: "", value: nil)
    @State private var password = ""
    
    @State private var submitState = ButtonState.disabled
    @State private var inputState = InputState.idle
    @State private var inputRoleState = InputState.idle
    @State private var roleOptions = [
        InputOption(key: "Petugas", value: UserRole.officer),
        InputOption(key: "Manager", value: UserRole.manager),
        InputOption(key: "Super admin", value: UserRole.super_admin),
    ]
    
    let user: ViewUser?
    
    init(_ user: ViewUser? = nil) {
        self.user = user
    }
    
    var request: UserRequest? {
        if self.number.isEmpty {
            return nil
        }
        
        if self.name.isEmpty {
            return nil
        }
        
        guard let role = self.role.value else {
            return nil
        }
        
        return UserRequest(
            number: self.number,
            password: self.password,
            name: self.name,
            role: role,
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
                
                if let user = self.user {
                    if user.role != .super_admin && user.id != self.network.authentication?.user.id {
                        ButtonIcon("trash", type: .secondary) {
                            self.submitState = .loading
                            self.userManager.delete(self.network, user_id: user.id) { user, err in
                                let _ = app.path.popLast()
                                self.submitState = .idle
                            }
                        }
                    }
                }
            },
            content: {
                InputText("Masukkan NIK", label: "NIK", state: $inputState, text: $number)

                InputText("Masukkan nama petugas", label: "Nama petugas", state: $inputState, text: $name)
                InputTextSelect("Pilih hak akses", label: "Hak akses", state: $inputRoleState, model: $role, options: $roleOptions)
                    .zIndex(1.5)
                InputText("Masukkan password", label: "Password", state: $inputState, text: $password, type: .secure)

            },
            footer: {
                ButtonRegular("SIMPAN", state: $submitState) {
                    if let request {
                        self.submitState = .loading
                        if let user = self.user {
                            // update
                            self.userManager.update(self.network, user_id: user.id, p: request) { user, err in
                                let _ = app.path.popLast()
                                self.submitState = .idle
                            }
                        } else {
                            // create
                            self.userManager.create(self.network, p: request ) { user, err in
                                let _ = app.path.popLast()
                                self.submitState = .idle
                            }
                        }
                    }
                }
            },
            title: self.user != nil ? "Informasi Petugas" : "Tambah Petugas"
        )
        .onChange(of: self.request) {
            if self.request != nil {
                self.submitState = .idle
            } else {
                self.submitState = .disabled
            }
        }
        .onAppear {
            guard let user = self.user else {
                return
            }
            guard let issuer = self.network.authentication?.user else {
                return
            }
            if user.role == .super_admin && issuer.id != user.id {
                self.inputState = .disabled
            }
            self.number = user.number
            self.name = user.name
            if let role = self.roleOptions.first(where: { $0.value == user.role }) {
                if let a = role.value {
                    if a == .super_admin {
                        self.inputRoleState = .disabled
                    }
                }
                self.role = role
            }
        }
    }
}



