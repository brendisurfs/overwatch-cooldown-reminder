use rdev::{listen, EventType};

use crate::CooldownMsg;

pub async fn handle_events(tx: tokio::sync::watch::Sender<CooldownMsg>) {
    // sends to the channel if its an e event.
    tokio::task::spawn_blocking(move || {
        listen(move |event| match event.event_type {
            EventType::KeyPress(key) => {
                if key == rdev::Key::KeyE {
                    tx.send(CooldownMsg::NoCooldown).unwrap();
                }
            }
            _ => (),
        })
        .expect("error listening to keyboard");
    });
}
