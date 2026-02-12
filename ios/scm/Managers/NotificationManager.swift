//
//  NotificationManager.swift
//  scm
//
//  Created by Kathlyne Sarah on 22/10/24.
//
import SwiftUI
import UserNotifications
import UIKit

@Observable
class NotificationManager {
    var device: String? {
        didSet {
            let defaults = UserDefaults.standard
            
            if let device = self.device {
                defaults.set(device, forKey: "device")
            } else {
                defaults.removeObject(forKey: "device")
            }
        }
    }
    var state: NotificationState  {
        didSet {
            let defaults = UserDefaults.standard
            
            defaults.set(self.state.rawValue, forKey: "notification")
        }
    }
    var requesting: Bool
    
    init() {
        self.state = .undefined
        self.requesting = false
    }
    
    func reject() {
        self.state = .disabled
    }
    func accept() {
        self.state = .enabled
    }
    func isPermitted(f: @escaping (Bool) -> Void) -> Void {
        let notificationCenter = UNUserNotificationCenter.current()
        notificationCenter.getNotificationSettings { settings in
            var permitted = false
            if settings.authorizationStatus == .authorized {
                permitted = true
            }
            
            f(permitted)
        }
    }
    func checkPermission(f: @escaping (NotificationState) -> Void) {
        let defaults = UserDefaults.standard
        
        let state: NotificationState? = if let a = defaults.string(forKey: "notification"), let b = NotificationState(rawValue: a) {
            b
        } else {
            nil
        }
        
        let notificationCenter = UNUserNotificationCenter.current()
        notificationCenter.getNotificationSettings { settings in
            switch settings.authorizationStatus {
                case .authorized:
                    print("Notification authorized.")
                    
                    if let state, state == .enabled || state == .disabled {
                        self.state = state
                    } else {
                        self.state = .enabled
                    }
                case .denied:
                    print("Notification permission denied.")
                    self.state = .unauthorized
                case .notDetermined:
                    print("Requesting notification permission...")
                    
                    if let state, state == .disabled {
                        self.state = state
                    } else {
                        self.state = .undefined
                    }
                default:
                    if let state, state == .disabled {
                        self.state = state
                    } else {
                        self.state = .undefined
                    }
            }
            
            f(self.state)
        }
    }
    func requestPermission(f: @escaping (NotificationState) -> Void) {
        let notificationCenter = UNUserNotificationCenter.current()
        notificationCenter.requestAuthorization(options: [.alert, .sound]) { granted, error in
            if let error {
                print("Error requesting authorization: \(error)")
            }
            if granted {
                self.state = .enabled
                print("Notification permission granted.")
            } else {
                self.state = .unauthorized
                print("Notification permission not granted.")
            }
            
            f(self.state)
        }
    }
    
    func subscribe(_ network: Network, f: @escaping ((Bool?, Error?) -> Void)) -> Void {
        guard let id = network.authentication?.user.id else {
            f(nil, "UNAUTHORIZED" as? Error)
            return
        }
        
        let defaults = UserDefaults.standard
        
        guard let token = defaults.string(forKey: "token") else {
            f(nil, "INVALID_TOKEN" as? Error)
            return
        }
        
        let p = NotificationSubscriberRequest(
            user_id: id,
            kind: NotificationSubscriberKind(apple: token)
        )
        
        let path = "/subscribers"
        
        guard let data = try? JSONEncoder().encode(p) else {
            f(nil, "PAYLOAD_INVALID" as? Error)
            return
        }
        
        network.req(path, method: .post, data: data) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response = response as? HTTPURLResponse, let data {
                print("Response: ", response)
                if response.statusCode == 201 {
                    guard let decoded = try? JSONDecoder().decode(NotificationSubscriber.self, from: data) else {
                        f(false, nil)
                        return
                    }
                    
                    self.device = decoded.id
                    
                    f(true, nil)
                } else {
                    f(false, nil)
                }
            } else {
                f(nil, "REQUEST_FAILED" as? Error)
            }
        }
    }
    
    func unsubscribe(_ network: Network, f: @escaping ((Bool?, Error?) -> Void)) -> Void {
        let defaults = UserDefaults.standard
        
        guard let token = defaults.string(forKey: "token") else {
            f(nil, "INVALID_TOKEN" as? Error)
            return
        }
        
        let path = "/subscribers?kind=apple&token=\(token)"
        
        network.req(path, method: .delete) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response = response as? HTTPURLResponse {
                print("Response: ", response)
                if response.statusCode == 204 {
                    f(true, nil)
                } else {
                    f(false, nil)
                }
            } else {
                f(nil, "REQUEST_FAILED" as? Error)
            }
        }
    }
    
    private func refresh(_ network: Network, subscriber_id: String, f: @escaping ((Bool?, Error?) -> Void)) -> Void {
        guard let id = network.authentication?.user.id else {
            f(nil, "UNAUTHORIZED" as? Error)
            return
        }
        
        let defaults = UserDefaults.standard
        
        guard let token = defaults.string(forKey: "token") else {
            f(nil, "INVALID_TOKEN" as? Error)
            return
        }
        
        let p = NotificationSubscriberRequest(
            user_id: id,
            kind: NotificationSubscriberKind(apple: token)
        )
        
        let path = "/subscribers/\(subscriber_id)"
        
        guard let data = try? JSONEncoder().encode(p) else {
            f(nil, "PAYLOAD_INVALID" as? Error)
            return
        }
        
        network.req(path, method: .put, data: data) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response = response as? HTTPURLResponse, let data {
                print("Response: ", response)
                if response.statusCode == 201 {
                    guard let decoded = try? JSONDecoder().decode(NotificationSubscriber.self, from: data) else {
                        f(false, nil)
                        return
                    }
                    
                    self.device = decoded.id
                    
                    f(true, nil)
                } else {
                    f(false, nil)
                }
            } else {
                f(nil, "REQUEST_FAILED" as? Error)
            }
        }
    }

    enum NotificationState: String {
        case enabled
        case disabled
        case unauthorized
        case undefined
    }
    struct NotificationSubscriber: Decodable {
        var id: String
        var user_id: String
        var kind: NotificationSubscriberKind
    }
    struct NotificationSubscriberRequest: Encodable {
        var user_id: String
        var kind: NotificationSubscriberKind
    }
    struct NotificationSubscriberKind: Codable {
        var apple: String
    }
}
