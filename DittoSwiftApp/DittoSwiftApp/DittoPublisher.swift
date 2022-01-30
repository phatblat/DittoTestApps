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

    init() {
        let ditto = Ditto()
//        try! ditto.setLicenseToken("my license token")
        try! ditto.tryStartSync()
    }


//    func receive<S>(subscriber: S) where S : Subscriber -> () {}
    func receive<S: Subscriber>(
                subscriber: S
            ) where S.Input == Output, S.Failure == Failure {
    }

//    func subscribe(_ subscriber: Subscriber) {}




//    func subscribe(_ subscriber: Subscriber) -> AnyCancellable {}
}
