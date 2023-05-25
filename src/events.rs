use rdev::{listen, EventType};
use tokio::sync::watch::Sender;

use crate::CooldownMsg;

pub async fn handle_events(tx: Sender<CooldownMsg>) {
    // sends to the channel if its an e event.
    tokio::task::spawn_blocking(move || {
        listen(move |event| match event.event_type {
            EventType::KeyPress(key) => {
                if key == rdev::Key::KeyE {
                    tx.send_if_modified(|k| {
                        if *k == CooldownMsg::HasCooldown {
                            *k = CooldownMsg::NoCooldown;
                            return true;
                        }
                        false
                    });
                }
            }
            // test send on release
            EventType::KeyRelease(key) => {
                if key == rdev::Key::KeyE {
                    tx.send(CooldownMsg::HasCooldown).unwrap();
                }
            }
            _ => (),
        })
        .expect("error listening to keyboard");
    });
}
