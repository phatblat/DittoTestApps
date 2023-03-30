//
// main.rs
// rust ditto-test-app
//

extern crate dotenv;

use std::{
    boxed::Box,
    env,
    error::Error,
    fs::File,
    str::FromStr,
    time::Instant,
    result::Result,
};
use dittolive_ditto::{
    prelude::*,
    store::{
        ditto_attachment_fetch_event::DittoAttachmentFetchEvent,
        ditto_attachment_fetcher::DittoAttachmentFetcher,
        ditto_attachment_token::DittoAttachmentToken,
    },
};
use dotenv::dotenv;


// Fetch attachment - called from update and insertion events in main closure
// Arguments - the collection (collection), the attachment token
// (attachment_token) and the fetchers vector which keeps the fetch_attachment
// callback alive
fn fetch_attachment(
    collection: &Collection,
    attachment_token: DittoAttachmentToken,
    fetchers: &mut Vec<DittoAttachmentFetcher>,
) {
    let start = Instant::now();
    let fetcher = collection
        .fetch_attachment(attachment_token, move |event| {
            if let DittoAttachmentFetchEvent::Completed { attachment: att } = event {
                let duration = start.elapsed();
                let file = File::open(att.path()).unwrap();
                let len = file.metadata().unwrap().len();
                let _speed = (len / 1024) as f32 / duration.as_secs_f32();
                // ::log::info!("---------------");
                // ::log::info!(
                //     "Attachment received - Time elapsed in fetch_attachment is: {:?}ms, speed \
                //      {:?}KB/s (size: {:?}KB)",
                //     duration.as_millis(),
                //     speed,
                //     len / 1024
                // );
                // ::log::info!("---------------");
            }
        })
        .unwrap();
    // So we see results
    fetchers.push(fetcher);
}

// fn main() {
fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let _license = env::var("DITTO_LICENSE").expect("$DITTO_LICENSE is not set");
    // eprintln!("DITTO_LICENSE: {}", license);

    let ditto = Ditto::builder()
        .with_temp_dir()
        .with_identity(|ditto_root| {
            // expected by other apps
            let app_id = AppId::from_str("e1ebf2c0-4aea-4fb7-85e9-b3d3ab7c0753")?;
            // We don't want a fully random OfflinePlayground Identity as we
            // need to make sure we have a specific AppId shared by all peers
            identity::OfflinePlayground::new(ditto_root, app_id)
        })?
        .with_minimum_log_level(CLogLevel::Info)
        .with_transport_config(|_identity| -> TransportConfig {
            let mut transport_config = TransportConfig::new();
            transport_config.listen.tcp.enabled = true;
            transport_config.listen.tcp.interface_ip = "0.0.0.0".to_string();
            transport_config.listen.tcp.port = 4040;
            transport_config.peer_to_peer.bluetooth_le.enabled = true;
            transport_config.peer_to_peer.lan.enabled = true;
            transport_config
        })?
        .build()?;

    // ditto.set_license_from_env(&String::from(license))?;
    ditto.set_license_from_env("DITTO_LICENSE")?;
    ditto.start_sync()?;

    // ::log::debug!("Ditto Cars App started!");

    let store = ditto.store();
    let collection = store.collection("cars").unwrap();
    let event_collection = collection.clone();
    // Used to keep attachment fetchers alive and printing results.
    let mut fetchers = vec![];

    let (tx, rx) = std::sync::mpsc::sync_channel(120);

    // This handler is called every time docs from local or remote sources are
    // committed to the local store which match the associated query.
    // `documents` is a vec of ALL documents matching the query after application of
    // the transaction.
    // `event` can be used to dissect out which of these are insertions.
    let event_handler = move |documents: Vec<BoxedDocument>, event| {
        // ::log::trace!(
        //     "Latency Receiver got {:?} with {} updated documents",
        //     &event,
        //     documents.len()
        // );
        match event {
            LiveQueryEvent::Initial { .. } => {
                // On an initial sync, we can calculate the latency for arrival of the first
                // document
                // ::log::info!("Initial Data loaded");
            }
            LiveQueryEvent::Update {
                insertions,
                updates,
                ..
            } => {
                // We only want to send the newest event
                for idx in insertions.iter().chain(updates.iter()) {
                    if let Some(car) = documents.get(*idx) {
                        let car_cbor = car.to_cbor().unwrap();
                        let _ = tx.send(car_cbor);
                        if let Ok(att_token) = car.get::<DittoAttachmentToken>("att") {
                            fetch_attachment(&event_collection, att_token, &mut fetchers);
                        }
                    }
                }
            }
        }
    };

    // downgrade our logging output before running the query
    Ditto::set_minimum_log_level(CLogLevel::Debug);
    // Find and report on all cars
    let _lq = collection.find_all().observe_local(event_handler);
    for car_cbor in rx.iter() {
        let car_str = serde_json::to_string(&car_cbor).unwrap();
        println!("{}", &car_str);
    }

    Ok(())
}
