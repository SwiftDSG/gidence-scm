//
//  Offline.swift
//  scm
//
//  Created by Nuzulul Salsabila on 09/10/24.
//

import SwiftUI

struct Offline: View {
    @Binding var buttonState: ButtonState
    
    private let VPW = UIScreen.main.bounds.size.width
    
    let reconnect: () -> Void
    
    init(_ state: Binding<ButtonState>, reconnect: @escaping () -> Void) {
        self._buttonState = state
        self.reconnect = reconnect
    }
        
    var body: some View {
        VStack(alignment: .leading, spacing: 24) {
            HStack {
                Image("offline")
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: VPW - 48)
            }
            
            VStack(alignment: .leading, spacing: 12) {
                TextHeadlineOne("Anda sedang offline")
                TextBodyFour("Sepertinya anda tidak terhubung dengan server kami, silahkan coba lagi", color: .fontSecondary)
            }
            
            ButtonRegular("COBA LAGI", state: $buttonState) {
                reconnect()
            }
        }
    }
}


