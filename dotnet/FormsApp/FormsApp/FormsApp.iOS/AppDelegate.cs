using DittoSDK;
using Ditto.Transports;
using UIKit;
using Foundation;
using System;

namespace FormsApp.iOS
{
    // The UIApplicationDelegate for the application. This class is responsible for launching the 
    // User Interface of the application, as well as listening (and optionally responding) to 
    // application events from iOS.
    [Register("AppDelegate")]
    public partial class AppDelegate : global::Xamarin.Forms.Platform.iOS.FormsApplicationDelegate
    {
        //
        // This method is invoked when the application has loaded and is ready to run. In this 
        // method you should instantiate the window, load the UI into it and then make the window
        // visible.
        //
        // You have 17 seconds to return from this method, or iOS will terminate your application.
        //
        public override bool FinishedLaunching(UIApplication app, NSDictionary options)
        {
            global::Xamarin.Forms.Forms.Init();
            LoadApplication(new App());

            NSError? error;
            NSFileManager fileManager = new NSFileManager();
            NSUrl url = fileManager.GetUrl(NSSearchPathDirectory.DocumentDirectory, NSSearchPathDomain.User, null, true, out error);
            if (error != null)
            {
                Console.WriteLine($"Error creating ditto directory: {error.LocalizedDescription}");
            }
            url.Append("ditto", true);

            string appId = "";
            string workingDir = url.Path;

            DittoIdentity identity = DittoIdentity.OfflinePlayground(appID: appId, workingDir: workingDir);

            var ditto = new DittoSDK.Ditto(identity, workingDir);

            return base.FinishedLaunching(app, options);
        }
    }
}
