//
//  Network.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 17/10/24.
//

import SwiftUI

//let API_BASE = String("://192.168.8.163:8000")
//let API_BASE = String("://172.20.10.4:8000")
//let API_BASE = String("://192.168.50.212:8000")
//let API_BASE = String("s://andrajayapasti.com/api")
let API_BASE = String("://100.85.142.81:8080")

@Observable
class Network {
    var authentication: UserAuthentication? {
        didSet {
            let defaults = UserDefaults.standard
            if let authentication = self.authentication, let data = try? JSONEncoder().encode(authentication) {
                defaults.set(data, forKey: "authentication")
            } else {
                defaults.removeObject(forKey: "authentication")
            }
        }
    }
    
    var state: NetworkWebSocketState
    private var ws: URLSessionWebSocketTask?
    
    init() {
        self.state = .offline
        self.ws = nil
        
        let defaults = UserDefaults.standard
        if let data = defaults.data(forKey: "authentication"), let authentication = try? JSONDecoder().decode(UserAuthentication.self, from: data) {
            self.authentication = authentication
        } else {
            self.authentication = nil
        }
    }
    
    func load(_ name: String, f: @escaping (Data?, URLResponse?, Error?) -> Void) -> Void {
        if let url = URL(string: "http\(API_BASE)/static/\(name)") {
            let request = URLRequest(url: url)
            
            if let cachedResponse = URLCache.shared.cachedResponse(for: request) {
                f(cachedResponse.data, cachedResponse.response, nil)
            } else {
                let task = URLSession.shared.dataTask(with: request) { (data, response, error) in
                    if response is HTTPURLResponse {
                        f(data, response, error)
                    } else {
                        f(data, response, error)
                    }
                }
                
                task.resume()
            }
        } else {
            f(nil, nil, "INVALID_URL" as? Error)
        }
    }
    
    func req(_ path: String, method: NetworkRequestMethod, data: Data? = nil, type: String = "application/json", f: @escaping (Data?, URLResponse?, Error?) -> Void) -> Void {
        if let url = URL(string: "http\(API_BASE)\(path)") {
            var request = URLRequest(url: url)
            
            switch method {
                case .get:
                    request.httpMethod = "GET"
                case .delete:
                    request.httpMethod = "DELETE"
                case .post:
                    request.httpMethod = "POST"
                case .put:
                    request.httpMethod = "PUT"
            }
            request.setValue(type, forHTTPHeaderField: "Content-Type")
            request.httpBody = data
            
            if let authentication = self.authentication {
                request.addValue("Bearer \(authentication.atk)", forHTTPHeaderField: "Authorization")
            }
            
            let task = URLSession.shared.dataTask(with: request) { (data, response, error) in
                if response is HTTPURLResponse {
                    f(data, response, error)
                } else {
                    f(data, response, error)
                }
            }
            
            task.resume()
        } else {
            f(nil, nil, "INVALID_URL" as? Error)
        }
    }
    
    func connect() {
        guard let url = URL(string: "ws\(API_BASE)/ws") else { return }
        let request = URLRequest(url: url)
        ws = URLSession.shared.webSocketTask(with: request)
        ws?.resume()
        
        self.state = .connecting
        self.listen()
        
        DispatchQueue.global().asyncAfter(deadline: .now() + 10) {
            if self.state == .connecting {
                self.state = .offline
                guard let ws = self.ws else { return }
                ws.cancel(with: .protocolError, reason: nil)
                self.ws = nil
            }
        }
    }
    
    func stream() -> AsyncStream<String> {
        AsyncStream<String> { [weak self] in
            guard let self else {
                // Self is gone, return nil to end the stream
                return nil
            }
            
            guard let message = try? await self.receive() else {
                return nil
            }
            
            // End the stream (by returning nil) if the calling Task was canceled
            return Task.isCancelled ? nil : message
        }
    }
    
    func send(_ message: NetworkWebSocketRequest) {
        guard let ws = self.ws else {
            return
        }
        
        let payload = switch message {
            case let .connect(user_id):
                """
                {"connect": "\(user_id)"}
                """
            case .disconnect:
                "disconnect"
        }
        
        
        ws.send(.string(payload)) { error in
            if let error = error {
                print(error.localizedDescription)
            }
        }
    }
    
    private func receive() async throws -> String? {
        guard let ws else {
            return nil
        }
        
        let message = try await ws.receive()
        self.state = .online
        
        if case let .string(text) = message {
            return text
        }
        return nil
    }
    
    private func listen() {
        guard let ws else {
            return
        }
        ws.sendPing() { err in
            if let err {
                print("CONN ERROR: \(err)")
                self.state = .offline
                ws.cancel(with: .protocolError, reason: nil)
                self.ws = nil
            } else {
                DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
                    self.listen()
                }
            }
        }
    }
    
    enum NetworkRequestMethod {
        case get
        case post
        case put
        case delete
    }
    enum NetworkWebSocketState {
        case online
        case offline
        case connecting
    }
}

typealias NetworkWebSocketMessageData = [ String: Int ]
struct NetworkWebSocketMessage: Decodable {
    var evidence: [ViewEvidence]?
    var processor: [String: Int]?
}
enum NetworkWebSocketRequest {
    case connect(String)
    case disconnect
}
