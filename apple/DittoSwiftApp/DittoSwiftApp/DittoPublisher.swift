//
//  DittoPublisher.swift
//  DittoSwiftApp
//
//  Created by Ben Chatelain on 1/30/22.
//

import DittoSwift
import Combine

class DittoPublisher: Publisher {
    typealias Output = Ditto
    typealias Failure = Error

    var ditto: Ditto?

    init() {
        let identity: DittoIdentity = .onlineWithAuthentication(appID: "46e08559-4388-4896-ba19-c1573509bb29", authenticationDelegate: self)
        ditto = Ditto(identity: identity)
        debugPrint("persistenceDirectory: \(ditto!.persistenceDirectory)")
ditto?.auth?.logout { ditto in
    let auth_dir = ditto.persistenceDirectory.appending(component: "ditto_auth")
    do {
        try FileManager.default.removeItem(at: auth_dir)
    } catch {
        debugPrint("Failed to remove directory: \(auth_dir.path). Error: \(error.localizedDescription)")
    }
}
ditto = nil
ditto = Ditto(identity: identity)
        try! ditto?.startSync()
    }


//    func receive<S>(subscriber: S) where S : Subscriber -> () {}
    func receive<S: Subscriber>(
                subscriber: S
            ) where S.Input == Output, S.Failure == Failure {
    }

//    func subscribe(_ subscriber: Subscriber) {}


//    func subscribe(_ subscriber: Subscriber) -> AnyCancellable {}
}

extension DittoPublisher: DittoAuthenticationDelegate {
    func authenticationRequired(authenticator: DittoSwift.DittoAuthenticator) {
        debugPrint("authenticationRequired")
    }

    func authenticationExpiringSoon(authenticator: DittoSwift.DittoAuthenticator, secondsRemaining: Int64) {
        debugPrint("authenticationExpiringSoon")
    }
}
