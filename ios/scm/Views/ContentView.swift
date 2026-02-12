//
//  ContentView.swift
//  scm
//
//  Created by Kemal Dwi Heldy Muhammad on 02/10/24.
//

import SwiftUI

struct ContentView: View {
    @Environment(\.safeAreaInsets) private var safeAreaInsets
    
    @State private var app: Application
    @State private var network: Network
    @State private var viewManager: ViewManager
    @State private var clusterManager: ClusterManager
    @State private var processorManager: ProcessorManager
    @State private var evidenceManager: EvidenceManager
    @State private var userManager: UserManager
    @State private var notificationManager: NotificationManager

    @State private var networkState = (1.0, 0.0)
    @State private var loginState = (1.0, 0.0)
    @State private var notificationState = (1.0, 0.0)

    @State private var initState = true
    @State private var buttonState = ButtonState.idle
    @State private var errorState: String? = nil
    
    @State private var task: Task<(), Never>? = nil
    
    private let VPW = UIScreen.main.bounds.size.width
    private let VPH = UIScreen.main.bounds.size.height

    init() {
        self.app = Application()
        self.network = Network()
        self.viewManager = ViewManager()
        self.clusterManager = ClusterManager()
        self.processorManager = ProcessorManager()
        self.evidenceManager = EvidenceManager()
        self.userManager = UserManager()
        self.notificationManager = NotificationManager()
    }
       
    var body: some View {
        ZStack {
            NavigationStack(path: self.$app.path) {
                Group {
                    if app.page == .EvidenceList {
                        EvidenceList()
                    } else if app.page == .ClusterList {
                        ClusterList()
                    } else if app.page == .UserList {
                        UserList()
                    } else {
                        EmptyView()
                    }
                }
                .navigationDestination(for: PageKind.self) { i in
                    if case let PageKind.ClusterForm(cluster) = i {
                        ClusterForm(cluster)
                    } else if case let PageKind.ClusterDetail(cluster) = i {
                        ClusterDetail(cluster)
                    } else if case let PageKind.UserForm(user) = i {
                        UserForm(user)
                    } else if i  == PageKind.Setting {
                        Setting()
                    } else if case let PageKind.SettingUser(user) = i {
                        SettingUser(user)
                    } else if case let PageKind.EvidenceDetail(evidence) = i {
                        EvidenceDetail(evidence)
                    }
                }
            }
            
            VStack {
                Spacer()
                TabBar()
            }
            .padding(.bottom, self.safeAreaInsets.bottom)
            .ignoresSafeArea()

            if self.initState {
                Splash()
            }
        }
        .environment(self.app)
        .environment(self.network)
        .environment(self.viewManager)
        .environment(self.clusterManager)
        .environment(self.processorManager)
        .environment(self.evidenceManager)
        .environment(self.userManager)
        .environment(self.notificationManager)
        .overlay(
            Group {
                if let image = self.app.image {
                    SCMImageFullScreen(image) {
                        self.app.image = nil
                    }
                } else {
                    EmptyView()
                }
            }
        )
        .overlay(
            ZStack {
                Rectangle()
                    .fill(.ultraThinMaterial)
                    .ignoresSafeArea()
                    .opacity(networkState.0)
                
                VStack(spacing: 0) {
                    Spacer()
                    
                    Offline($buttonState) {
                        self.buttonState = .loading
                        self.network.connect()
                    }
                    .padding(24)
                    .frame(
                        maxWidth: .infinity,
                        alignment: .topLeading
                    )
                    .background(
                        Rectangle()
                            .fill(.backgroundOne)
                            .clipShape(.rect(
                                topLeadingRadius: 24,
                                bottomLeadingRadius: 0,
                                bottomTrailingRadius: 0,
                                topTrailingRadius: 24
                            ))
                            .ignoresSafeArea()
                    )
                    .offset(y: networkState.1)
                }
            }
        )
        .overlay(
            ZStack {
                Rectangle()
                    .fill(.ultraThinMaterial)
                    .ignoresSafeArea()
                    .opacity(loginState.0)
                
                VStack(spacing: 0) {
                    Spacer()
                    
                    Login(self.$buttonState) { request in
                        hideKeyboard()
                        self.buttonState = .loading
                        self.errorState = nil
                        self.userManager.login(self.network, p: request) { response, err in
                            if let response {
                                self.network.authentication = response
                            } else if let err {
                                print("Error: \(err)")
                                self.buttonState = .idle
                                self.errorState = "NIP atau password salah"
                            }
                        }
                    }
                    .padding(24)
                    .frame(
                        maxWidth: .infinity,
                        alignment: .topLeading
                    )
                    .background(
                        ZStack(alignment: .topLeading) {
                            Rectangle()
                                .fill(.error)
                                .frame(width: VPW, height: 48)
                                .clipShape(.rect(
                                    topLeadingRadius: 24,
                                    bottomLeadingRadius: 0,
                                    bottomTrailingRadius: 0,
                                    topTrailingRadius: 24
                                ))
                                .overlay(HStack { TextHeadlineFive(self.errorState ?? "NIP atau password salah", color: .fontPrimary) }.frame(height: 24), alignment: .top)
                                .offset(y: self.errorState != nil ? -24 : 0)
                                .animation(.spring(duration: 0.25), value: self.errorState != nil)
                            Rectangle()
                                .fill(.backgroundOne)
                                .clipShape(.rect(
                                    topLeadingRadius: 24,
                                    bottomLeadingRadius: 0,
                                    bottomTrailingRadius: 0,
                                    topTrailingRadius: 24
                                ))
                                .ignoresSafeArea()
                        }
                    )
                    .offset(y: self.loginState.1)
                }
            }
        )
        .overlay(
            ZStack {
                Rectangle()
                    .fill(.ultraThinMaterial)
                    .ignoresSafeArea()
                    .opacity(self.notificationState.0)
                
                VStack(spacing: 0) {
                    Spacer()
                    
                    Notification(
                        accept: {
                            self.notificationManager.requestPermission() { permission in
                                self.notificationManager.requesting = false
                                if permission == .enabled {
                                    self.notificationManager.subscribe(self.network) { success, _ in
                                        print("SUBSCRIBE STATUS: \(success)")
                                    }
                                }
                            }
                        },
                        reject: {
                            self.notificationManager.requesting = false
                            self.notificationManager.unsubscribe(self.network) { success, _ in
                                if let success, success {
                                    self.notificationManager.reject()
                                }
                            }
                        }
                    )
                        .padding(24)
                        .frame(
                            maxWidth: .infinity,
                            alignment: .topLeading
                        )
                        .background(
                            Rectangle()
                                .fill(.backgroundOne)
                                .clipShape(.rect(
                                    topLeadingRadius: 24,
                                    bottomLeadingRadius: 0,
                                    bottomTrailingRadius: 0,
                                    topTrailingRadius: 24
                                ))
                                .ignoresSafeArea()

                        )
                        .offset(y: self.notificationState.1)
                }
            }
        )
        .onAppear {
            self.networkState = (0.0, VPH)
            self.loginState = (0.0, VPH)
            self.notificationState = (0.0, VPH)

            self.network.connect()
        }
        .onChange(of: self.network.state) {
            if self.network.state == .offline {
                if self.loginState.0 == 1.0 {
                    withAnimation(.spring(duration: 0.5)) {
                        self.loginState.0 = 0.0
                        self.loginState.1 = VPH * 0.6
                    }
                }
                withAnimation(.spring(duration: 0.25)) {
                    self.networkState.0 = 1.0
                    self.networkState.1 = 0.0
                } completion: {
                    self.buttonState = .idle
                }
            } else if self.network.state == .online {
                withAnimation(.spring(duration: 0.5)) {
                    self.networkState.0 = 0.0
                    self.networkState.1 = VPH * 0.6
                } completion: {
                    self.buttonState = .idle
                }
                if let authentication = self.network.authentication {
                    self.userManager.refresh(self.network, p: UserRefreshRequest(authentication.rtk)) { response, err in
                        if let response {
                            self.network.authentication = response
                        } else {
                            self.network.authentication = nil
                            withAnimation(.spring(duration: 0.25)) {
                                self.loginState.0 = 1.0
                                self.loginState.1 = 0.0
                            }
                        }
                        if self.initState {
                            self.initState = false
                        }
                    }
                } else {
                    withAnimation(.spring(duration: 0.25)) {
                        self.loginState.0 = 1.0
                        self.loginState.1 = 0.0
                    }
                    if self.initState {
                        self.initState = false
                    }
                }
            } else {
                self.connect()
            }
        }
        .onChange(of: self.network.authentication) {
            if self.network.state == .online {
                if let user = self.network.authentication?.user {
                    self.notificationManager.checkPermission() { permission in
                        if permission == .undefined {
                            self.notificationManager.requesting = true
                        }
                    }
                    
                    self.network.send(NetworkWebSocketRequest.connect(user.id))
                    
                    withAnimation(.spring(duration: 0.5)) {
                        self.loginState.0 = 0.0
                        self.loginState.1 = VPH * 0.6
                    } completion: {
                        self.buttonState = .idle
                    }
                } else {
                    self.notificationManager.unsubscribe(self.network) { _, _ in
                        print("UNSUBBED")
                    }
                    self.network.send(NetworkWebSocketRequest.disconnect)
                    withAnimation(.spring(duration: 0.25)) {
                        self.loginState.0 = 1.0
                        self.loginState.1 = 0.0
                    } completion: {
                        self.buttonState = .idle
                    }
                }
            }
        }
        .onChange(of: self.notificationManager.requesting) {
            if self.notificationManager.requesting {
                withAnimation(.spring(duration: 0.25)) {
                    self.notificationState.0 = 1.0
                    self.notificationState.1 = 0.0
                }
            } else {
                withAnimation(.spring(duration: 0.5)) {
                    self.notificationState.0 = 0.0
                    self.notificationState.1 = VPH * 0.6
                }
            }
        }
    }
    
    private func connect() {
        if let task = self.task {
            task.cancel()
        }
       
        self.task = Task {
            await self.stream()
        }
    }
    
    private func stream() async {
        for try await message in self.network.stream() {
            print("WS MESSAGE RAW: \(message)")
            guard let data = message.data(using: .utf8) else {
                continue
            }
            
            guard let message = try? JSONDecoder().decode(NetworkWebSocketMessage.self, from: data) else {
                continue
            }
            
            print("WS MESSAGE: \(message)")
            if let data = message.evidence {
                self.viewManager.evidenceList.evidences.insert(contentsOf: data, at: 0)
            } else if let data = message.processor {
                self.processorManager.online = data
            }
        }
    }
}
