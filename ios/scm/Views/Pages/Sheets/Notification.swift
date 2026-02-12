import SwiftUI

struct Notification: View {
    let accept: () -> Void
    let reject: () -> Void
    
    var body: some View {
        VStack(alignment: .leading, spacing: 24) {
            VStack(alignment: .leading, spacing: 12) {
                TextHeadlineOne("Aktifkan notifikasi")
                TextBodyFour("Dengan mengaktifkan notifikasi, kamu dapat mendapatkan peringatan secara langsung apabila terjadi pelanggaran", color: .fontSecondary)
            }
            VStack(spacing: 12) {
                ButtonRegular("AKTIFKAN") {
                    self.accept()
                }
                ButtonRegular("NANTI SAJA", type: .quartenary) {
                    self.reject()
                }
            }
        }
    }
}

