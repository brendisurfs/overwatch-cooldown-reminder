pub mod events;
use events::handle_events;
use std::{cell::Cell, time::Duration};
use tokio::sync::watch::Receiver;

use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::PhysicalPosition, LogicalSize, WindowBuilder};

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

                .blink_me {
                    pointer-events: none;
                    animation: blinker 0.5s step-start infinite;
                  }
                  
                  @keyframes blinker {
                    50% {
                      opacity: 0;
                    }
                  }
            </style>
        "#
            .to_owned(),
        )
}

fn app(cx: Scope<AppProps>) -> Element {
    let status = use_state(cx, || CooldownMsg::HasCooldown);

    // handle turn off cooldown
    use_effect(cx, (status,), |status| async move {
        if status.0.get() == &CooldownMsg::NoCooldown {
            tokio::time::sleep(Duration::from_secs(5)).await;
            status.0.set(CooldownMsg::HasCooldown);
            println!("reset cooldown");
        }
    });
    // keypress handler.
    let _: &Coroutine<()> = use_coroutine(cx, |_| {
        let recv = cx.props.receiver.take();
        let status = status.to_owned();
        async move {
            if let Some(mut r) = recv {
                while r.changed().await.is_ok() {
                    let msg = *r.borrow();
                    if status.get() != &CooldownMsg::NoCooldown {
                        status.set(msg);
                        println!("Cooldown used.");
                    }
                }
            }
        }
    });

    cx.render(rsx! {
    div {
        width: "100%",
        class: "blink_me",
        color: match status.get() {
                    &CooldownMsg::HasCooldown => "red",
                    _ => "blue",
                },
        height: "500px",
        text_align: "center",
        font_size: "120px",
        background_color: "transparent",
        match status.get() {
            &CooldownMsg::HasCooldown => "USE YOUR COOLDOWNS",
            &CooldownMsg::NoCooldown => "",
        }
    }
    })
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(false)
        .with_focused(false)
        .with_resizable(false)
        .with_always_on_top(true)
        .with_min_inner_size(LogicalSize::new(2048, 200))
        .with_position(PhysicalPosition::new(0, 10))
        .with_max_inner_size(LogicalSize::new(2048, 200))
}
