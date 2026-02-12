//
//  ProcessorManager.swift
//  scm
//
//  Created by Billy Anthony Bingtoyo on 17/10/24.
//

import SwiftUI
import Foundation
import Network
import NIO

@Observable class ProcessorManager {
    var online: [String: Int]
    
    init() {
        self.online = [:]
    }
    
    func create(_ network: Network, _ p: ProcessorRequest, f: @escaping ((Processor?, Error?) -> Void)) -> Void {
        do {
            let path = "/processors"
            
            let data = try JSONEncoder().encode(p)
            
            network.req(path, method: .post, data: data) { (data, response, error) in
                if let error {
                    print("Error: ", error)
                    f(nil, error)
                    return
                }
                if let response {
                    print("Response: ", response)
                }
                if let data {
                    DispatchQueue.main.async {
                        do {
                            let decoded = try JSONDecoder().decode(Processor.self, from: data)
                            f(decoded, nil)
                        } catch let error {
                            print("Error: ", error)
                            f(nil, error)
                        }
                    }
                }
            }
        } catch let error {
            print("Error: ", error)
            f(nil, error)
        }
    }
    func delete(_ network: Network,  processor_id: String, f: @escaping ((Bool?, Error?) -> Void)) -> Void {
        let path = "/processors/\(processor_id)"
        
        network.req(path, method: .delete) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response {
                print("Response: ", response)
            }
            f(true, nil)
        }
    }
    func get(_ network: Network, _ processor_id: String, f: @escaping ((ViewProcessor?, Error?) -> Void)) -> Void {
        let path = "/processors/\(processor_id)"
        
        network.req(path, method: .get) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response {
                print("Response: ", response)
            }
            if let data {
                DispatchQueue.main.async {
                    do {
                        let decoded = try JSONDecoder().decode(ViewProcessor.self, from: data)
                        f(decoded, nil)
                    } catch let error {
                        print("Error: ", error)
                        f(nil, error)
                    }
                }
            }
        }
    }
    func getMany(_ network: Network, cluster_id: String? = nil, date_minimum: Int? = nil, date_maximum: Int? = nil, limit: Int? = nil, skip: Int? = nil, f: @escaping (([ViewProcessor]?, Error?) -> Void)) -> Void {
        var path = "/processors?"
        
        if let cluster_id {
            path += "cluster_id=\(cluster_id)&"
        }
        if let date_minimum {
            path += "date_minimum=\(date_minimum)&"
        }
        if let date_maximum {
            path += "date_maximum=\(date_maximum)&"
        }
        if let limit {
            path += "limit=\(limit)&"
        }
        if let skip {
            path += "skip=\(skip)&"
        }
        
        network.req(path, method: .get) { (data, response, error) in
            if let error {
                print("Error: ", error)
                f(nil, error)
                return
            }
            if let response {
                print("Response: ", response)
            }
            if let data {
                DispatchQueue.main.async {
                    do {
                        let decoded = try JSONDecoder().decode([ViewProcessor].self, from: data)
                        f(decoded, nil)
                    } catch let error {
                        print("Error: ", error)
                        f(nil, error)
                    }
                }
            }
        }
    }
    
    func scanCameras() async -> [String]? {
        var hosts: [String] = []
        
        let probe = """
        <?xml version="1.0" encoding="UTF-8"?>
        <e:Envelope xmlns:e="http://www.w3.org/2003/05/soap-envelope"
                    xmlns:w="http://schemas.xmlsoap.org/ws/2004/08/addressing"
                    xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery"
                    xmlns:dn="http://www.onvif.org/ver10/network/wsdl">
          <e:Header>
            <w:MessageID>uuid:\(UUID().uuidString)</w:MessageID>
            <w:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</w:To>
            <w:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</w:Action>
          </e:Header>
          <e:Body>
            <d:Probe>
              <d:Types>dn:NetworkVideoTransmitter</d:Types>
            </d:Probe>
          </e:Body>
        </e:Envelope>
        """
        
        guard let en0Interface = try? System.enumerateDevices().filter({ $0.name == "en0" && $0.broadcastAddress != nil }).first else {
            print("Failed to create en0 interface")
            return nil
        }
        
        guard let broadcastAddress = en0Interface.broadcastAddress else {
            print("Failed to get broadcast address")
            return nil
        }
        
        guard let address = en0Interface.address else {
            print("Failed to get address from en0 interface")
            return nil
        }
        
        let server = try? await DatagramBootstrap(group: NIOSingletons.posixEventLoopGroup)
            .channelOption(ChannelOptions.socket(SOL_SOCKET, SO_BROADCAST), value: 1)
            .bind(to: address)
            .flatMapThrowing { channel in
                return try NIOAsyncChannel(
                    wrappingChannelSynchronously: channel,
                    configuration:  NIOAsyncChannel.Configuration(
                        inboundType: AddressedEnvelope<ByteBuffer>.self,
                        outboundType: AddressedEnvelope<ByteBuffer>.self
                    )
                )
            }
            .get()
        
        guard let server else {
            print("Failed to create server channel")
            return nil
        }
        
        var destAddr = broadcastAddress
        destAddr.port = 3702
        
        let task = Task {
            try await server.executeThenClose { inbound, outbound in
                let data = ByteBuffer(string: probe)
                try await outbound.write(AddressedEnvelope(remoteAddress: destAddr, data: data))
                
                for try await packet in inbound {
                    guard let ip = packet.remoteAddress.ipAddress else {
                        continue
                    }
                    
                    if !hosts.contains(ip) {
                        hosts.append(ip)
                    }
                }
            }
        }
        
        let t = DispatchTime.now() + 1
        while !task.isCancelled {
            if DispatchTime.now() > t {
                task.cancel()
                _ = try? await server.channel.closeFuture.get()
            }
        }
        return hosts
    }
    
    func scanProcessor() async -> [(String, String)] {
        var host: [(String, String)] = []
        
        let server = try? await DatagramBootstrap(group: NIOSingletons.posixEventLoopGroup)
            .bind(host: "0.0.0.0", port: 34254)
            .flatMapThrowing { channel in
                return try NIOAsyncChannel(
                    wrappingChannelSynchronously: channel,
                    configuration:  NIOAsyncChannel.Configuration(
                        inboundType: AddressedEnvelope<ByteBuffer>.self,
                        outboundType: AddressedEnvelope<ByteBuffer>.self
                    )
                )
            }
            .get()
        
        guard let server else {
            print("Failed to create server channel")
            return host
        }
        
        let task = Task {
            try await server.executeThenClose { inbound, _ in
                for try await var packet in inbound {
                    guard let string = packet.data.readString(length: packet.data.readableBytes) else {
                        continue
                    }
                    
                    guard let ip = packet.remoteAddress.ipAddress else {
                        continue
                    }
                    
                    if !host.contains(where: { $0.0 == string }) {
                        host.append((string, ip))
                    }
                }
            }
        }
        
        let t = DispatchTime.now() + 1
        while !task.isCancelled {
            if DispatchTime.now() > t {
                task.cancel()
                _ = try? await server.channel.closeFuture.get()
            }
        }
        return host
    }
}
