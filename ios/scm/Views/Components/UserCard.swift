//
//  UserCard.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 29/10/24.
//

import SwiftUI

struct UserCard: View {
    let user: ViewUser
    let depth: CardDepth
    let open: (() -> Void)?
    let delete: (() -> Void)?
    let select: (() -> Void)?

    init(_ user: ViewUser, depth: CardDepth = .one, open: (() -> Void)? = nil, delete: (() -> Void)? = nil, select: (() -> Void)? = nil) {
        self.user = user
        self.depth = depth
        self.open = open
        self.delete = delete
        self.select = select
    }
    
    var body: some View {
        HCard(
            content: {
                if self.depth == .one {
                    VStack(alignment: .leading, spacing: 3) {
                        TextHeadlineFour(self.user.name)
                        TextBodyFour(self.user.role.toString(), color: .fontSecondary)
                    }
                } else {
                    VStack(alignment: .leading, spacing: 3) {
                        TextHeadlineFive(self.user.name)
                        TextBodyFour(self.user.role.toString(), color: .fontSecondary)
                    }
                }
                
                Spacer()
                if let open = self.open {
                    ButtonIcon("pencil", type: .secondary) {
                        open()
                    }
                }
                if let delete = self.delete {
                    ButtonIcon("xmark", type: .secondary) {
                        delete()
                    }
                }
                if let select = self.select {
                    ButtonIcon("plus", type: .secondary) {
                        select()
                    }
                }
            },
            depth: self.depth
        )
        .onTapGesture {
            if let open = self.open {
                open()
            }
        }
    }
}

