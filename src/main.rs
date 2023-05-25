pub mod events;
use events::handle_events;
use rdev::{listen, EventType};
use std::{cell::Cell, net::TcpListener};
use tokio::sync::watch::{Receiver, Sender};
use tungstenite::{accept, connect, Message};
use url::Url;

use dioxus::{
    html::input_data::keyboard_types::{Key, KeyboardEvent},
    prelude::*,
};
use dioxus_desktop::{
    tao::{dpi::PhysicalPosition, event_loop},
    LogicalSize, WindowBuilder,
};
use dioxus_signals::{use_init_signal_rt, use_signal};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CooldownMsg {
    HasCooldown,
    NoCooldown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    pub status: CooldownMsg,
}

impl State {
    pub fn set(&mut self, status: CooldownMsg) {
        self.status = status;
    }
}

pub struct AppProps {
    sender: Cell<Option<Sender<CooldownMsg>>>,
    receiver: Cell<Option<Receiver<CooldownMsg>>>,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::watch::channel(CooldownMsg::HasCooldown);
    tokio::spawn(async move {
        handle_events().await;
    });
    dioxus_desktop::launch_with_props(
        app,
        AppProps {
            sender: Cell::new(Some(tx)),
            receiver: Cell::new(Some(rx)),
        },
        make_config(),
    );
}

fn app(cx: Scope<AppProps>) -> Element {
    println!("re-render check");
    cx.render(rsx! {
        div {
            width: "100%",
            color: "red",
            height: "500px",
            font_size: "80px",
            text_align: "center",
            background_color: "transparent",
            cooldown_text {}

        }
    })
}

fn cooldown_text(cx: Scope) -> Element {
    use_init_signal_rt(cx);
    let text_state = use_signal(cx, || CooldownMsg::HasCooldown);

    let co: &Coroutine<()> = use_coroutine(cx, |rx| async move {
        let (mut socket, res) = connect(Url::parse("ws://localhost:7878").unwrap()).unwrap();
        let msg = socket.read_message().unwrap();
        println!("msg: {:?}", msg);
        if msg.is_text() {
            text_state.clone().set(CooldownMsg::NoCooldown);
            println!("{:?}", msg);
        }
    });
    cx.render(rsx! {
        div {
            format!("{:?}", text_state.read())
        }
    })
}

fn make_config() -> dioxus_desktop::Config {
    dioxus_desktop::Config::default()
        .with_window(make_window())
        .with_custom_head(
            r#"
            <style type="text/css">
                html, body {
                    height: 500px;
                    margin: 0;
                    overscroll-behavior-y: none;
                    overscroll-behavior-x: none;
                    overflow: hidden;
                }
                #main, #bodywrap {
                    height: 100%;
                    margin: 0;
                    overscroll-behavior-x: none;
                    overscroll-behavior-y: none;
                }
            </style>
        "#
            .to_owned(),
        )
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(false)
        .with_resizable(false)
        .with_always_on_top(true)
        .with_position(PhysicalPosition::new(0, 0))
        .with_max_inner_size(LogicalSize::new(100000, 100))
}
