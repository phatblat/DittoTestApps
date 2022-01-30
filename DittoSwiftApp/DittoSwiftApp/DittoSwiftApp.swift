//
//  DittoSwiftApp.swift
//  DittoSwiftApp
//
//  Created by Ben Chatelain on 1/30/22.
//

import SwiftUI

@main
struct DittoSwiftApp: App {
    @ObservedObject var dittoPublisher = DittoPublisher()

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
