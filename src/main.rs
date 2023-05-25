pub mod events;
use events::handle_events;
use std::{cell::Cell, time::Duration};
use tokio::sync::watch::{Receiver, Sender};
use tungstenite::connect;
use url::Url;

use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::PhysicalPosition, LogicalSize, WindowBuilder};
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

pub struct AppProps {
    pub receiver: Cell<Option<Receiver<CooldownMsg>>>,
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::watch::channel(CooldownMsg::HasCooldown);
    tokio::spawn(async move {
        handle_events(tx).await;
    });
    dioxus_desktop::launch_with_props(
        app,
        AppProps {
            receiver: Cell::new(Some(rx)),
        },
        make_config(),
    );
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

fn app(cx: Scope<AppProps>) -> Element {
    let status = use_state(cx, || CooldownMsg::HasCooldown);
    let _: &Coroutine<()> = use_coroutine(cx, |_| {
        let recv = cx.props.receiver.take();
        let status = status.to_owned();
        async move {
            if let Some(mut r) = recv {
                while r.changed().await.is_ok() {
                    let msg = *r.borrow();
                    status.set(msg);
                }
            }
        }
    });
    cx.render(rsx! {
        div {
            width: "100%",
            color: "red",
            height: "500px",
            font_size: "80px",
            text_align: "center",
            background_color: "transparent",
            "{status:?}"

        }
    })
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
