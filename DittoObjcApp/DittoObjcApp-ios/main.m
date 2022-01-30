//
//  main.m
//  DittoObjcApp-ios
//
//  Created by Ben Chatelain on 1/29/22.
//

#import "AppDelegate.h"
@import UIKit;

int main(int argc, char * argv[]) {
    NSString * appDelegateClassName;
    @autoreleasepool {
        // Setup code that might create autoreleased objects goes here.
        appDelegateClassName = NSStringFromClass([AppDelegate class]);
    }
    return UIApplicationMain(argc, argv, nil, appDelegateClassName);
}
