use std::net::TcpListener;

use rdev::{listen, EventType};
use tungstenite::accept;

use crate::CooldownMsg;

pub async fn handle_events() {
    let (tx, mut rx) = tokio::sync::watch::channel(CooldownMsg::HasCooldown);
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

    // tokio::task::spawn_blocking(move || {
    //     let server = TcpListener::bind("127.0.0.1:7878").unwrap();
    //     println!("websocket bound");
    //     for stream in server.incoming() {
    //         println!("incoming...");
    //         tokio::task::spawn_blocking(move || {
    //             let mut websocket = accept(stream.unwrap()).unwrap();
    //             println!("successfully connected to socket");

    //             loop {
    //                 let msg = websocket.read_message().unwrap();
    //                 if msg.is_binary() || msg.is_text() {
    //                     websocket.write_message(msg).unwrap();
    //                 }
    //             }
    //         });
    //     }
    // });

    // server/publisher
    tokio::task::spawn(async move {
        let server = TcpListener::bind("127.0.0.1:7878").unwrap();
        println!("websocket bound to 127.0.0.1:7878");
        while rx.changed().await.is_ok() {
            let msg = *rx.borrow();
            println!("msg received from channel = {:?}", msg);
            for stream in server.incoming() {
                tokio::task::spawn_blocking(move || {
                    let mut websocket = accept(stream.unwrap()).unwrap();
                    println!("successfully connected to socket");
                    websocket
                        .write_message(tungstenite::Message::Text(format!("{msg:?}")))
                        .unwrap();
                });
            }
            //sleep(Duration::from_secs(2)).await;
            // println!("reup Cooldown");
        }
    });
}

pub async fn connect_to_ws_server() {}
