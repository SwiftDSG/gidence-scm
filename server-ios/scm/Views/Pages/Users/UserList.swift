//
//  UserList.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 28/10/24.
//

import SwiftUI

struct UserList: View {
    @Environment(Application.self) private var app
    @Environment(Network.self) private var network
    @Environment(ViewManager.self) private var viewManager
    @Environment(UserManager.self) private var userManager

    @State private var loading = true
    @State private var search = ""
    
    @State private var users: [ViewUser] = []
    
    private var usersFilter: [ViewUser] {
        if search.isEmpty {
            return viewManager.userList.users
        } else {
            return viewManager.userList.users.filter { user in
                user.name.lowercased().contains(search.lowercased())
            }
        }
    }

    var body: some View {
        PageWrapper(
            header: {
                ButtonIcon("person", type: .secondary) {
                    self.app.path.append(.Setting)
                }
                
                Spacer()
                
                ButtonIcon("plus", type: .primary) {
                    self.app.path.append(.UserForm(nil))
                }
            },
            content: {
                VStack(spacing: 18) {
                    if self.loading {
                        ProgressView()
                    } else {
                        if usersFilter.isEmpty {
                            VStack(spacing: 24) {
                                Image("search.empty")
                                    .resizable()
                                    .aspectRatio(contentMode: .fit)
                                    .frame(maxWidth: .infinity)
                                
                                VStack(spacing: 3) {
                                    TextHeadlineFour("Data masih kosong")
                                    TextBodyFour("Tidak ada petugas yang ditemukan", color: .fontSecondary)
                                }
                            }
                            .padding(.top, 24)
                        } else {
                            InputText("Cari nama petugas...", text: $search)

                            ForEach(self.usersFilter, id: \.self) { user in
                                if network.authentication?.user.id == user.id {
                                    UserCard(user)
                                } else {
                                    UserCard(
                                        user,
                                        open: {
                                            self.app.path.append(.UserForm(user))
                                        }
                                    )
                                }
                            }
                        }
                        Rectangle()
                            .fill(.clear)
                            .frame(height: 48)
                    }
                }
            },
            title: "Daftar petugas"
        )
        .onAppear {
            if self.viewManager.userList.users.isEmpty {
                self.loading = true
            } else {
                self.loading = false
            }
            
            self.userManager.getMany(self.network) { users, err in
                self.loading = false
                if var users {
                    if var user = self.network.authentication?.user {
                        if let index = users.firstIndex(where: { $0.id == user.id }) {
                            users.remove(at: index)
                        }
                        user.name = "\(user.name) (Saya)"
                        users.insert(user, at: 0)
                    }
                    self.viewManager.userList.users = users
                }
            }
        }
    }
}
