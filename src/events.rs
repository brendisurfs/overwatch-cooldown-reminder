use rdev::{listen, EventType};

use crate::CooldownKeys;

// handles our keyboard events, sends them back up the channel.
pub async fn handle_events(tx: tokio::sync::watch::Sender<CooldownKeys>) {
    // sends to the channel if its an e event.
    tokio::task::spawn_blocking(move || {
        listen(move |event| match event.event_type {
            EventType::KeyPress(key) => {
                if key == rdev::Key::KeyE {
                    tx.send(CooldownKeys::EKey).expect("key to be sent");
                }
                if key == rdev::Key::ShiftLeft {
                    tx.send(CooldownKeys::ShiftKey)
                        .expect("shift key to be sent");
                }
            }
            _ => (),
        })
        .expect("error listening to keyboard");
    });
}
