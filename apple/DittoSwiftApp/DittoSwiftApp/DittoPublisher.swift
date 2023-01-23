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
        ditto = Ditto()
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
