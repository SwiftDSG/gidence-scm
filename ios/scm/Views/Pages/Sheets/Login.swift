//
//  Login.swift
//  scm
//
//  Created by Nuzulul Salsabila on 09/10/24.
//

import SwiftUI

struct Login: View {
    @Binding var buttonState: ButtonState
    
    @State private var number: String = ""
    @State private var password: String = ""
    @State private var inputState = InputState.idle
    
    let submit: (UserAuthenticationRequest) -> Void
    
    init(_ state: Binding<ButtonState>, submit: @escaping (UserAuthenticationRequest) -> Void) {
        self._buttonState = state
        self.submit = submit
    }
    
    var body: some View {
        VStack(alignment: .leading, spacing: 24) {
            VStack(alignment: .leading, spacing: 12) {
                TextHeadlineOne("Masuk ke scm")
                TextBodyFour("Silahkan isi data login", color: .fontSecondary)
            }
            VStack(spacing: 48) {
                VStack(spacing: 24) {
                    InputText("Nomor Induk Pegawai", label: "NIP", text: $number)
                    InputText("Masukkan Kata Sandi", label: "Kata Sandi", text: $password, type: .secure)
                }
                ButtonRegular("MASUK", state: $buttonState) {
                    submit(UserAuthenticationRequest(number: self.number, password: self.password))
                }
            }
        }
    }
}


