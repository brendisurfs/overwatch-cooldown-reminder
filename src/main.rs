pub mod audio;
pub mod events;
use events::handle_events;
use std::{cell::Cell, time::Duration};
use tokio::sync::watch::Receiver;

use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::PhysicalPosition, LogicalSize, WindowBuilder};

use crate::audio::play_audio_idk;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CooldownMsg {
    HasCooldown,
    NoCooldown,
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

    let init_state = AppProps {
        receiver: Cell::new(Some(rx)),
    };
    dioxus_desktop::launch_with_props(app, init_state, make_config());
}

fn make_config() -> dioxus_desktop::Config {
    dioxus_desktop::Config::default()
        .with_window(make_window())
        .with_custom_head(include_str!("../style.html").to_string())
}

fn app(cx: Scope<AppProps>) -> Element {
    let status = use_state(cx, || CooldownMsg::HasCooldown);
    // handle turn off cooldown
    use_effect(cx, (status,), |status| async move {
        if status.0.get() == &CooldownMsg::NoCooldown {
            tokio::time::sleep(Duration::from_secs(8)).await;
            status.0.set(CooldownMsg::HasCooldown);
            tokio::task::spawn_blocking(move || {
                play_audio_idk();
            });
            println!("reset cooldown.");
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

    let style = r#"
        display: flex;
        flex-direction: row;
    "#;

    cx.render(rsx! {
        div {
            color: "red",
            width: "100%",
            height: "300px",
            font_size: "140px",
            text_align: "center",
            background_color: "transparent",
            match status.get() {
                &CooldownMsg::HasCooldown => {
                    cx.render(rsx! {
                        div  {
                                class: "blinking_text",
                                style: "{style}",
                                image_component {}
                                    div {
                                        "USE YOUR COOLDOWNS"
                                    }
                                image_component {}
                            }
                        })
                },
                &CooldownMsg::NoCooldown => cx.render(rsx! {
                    div {}
                }),
            }
    }
    })
}

fn image_component(cx: Scope) -> Element {
    let image_path = "assets/image.jpg";

    cx.render(rsx! {
        div {
            img {
                src: "{image_path}",
                width: "200px",
                height: "180px"
            }
        }
    })
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_focused(false)
        .with_resizable(false)
        .with_transparent(true)
        .with_decorations(false)
        .with_always_on_top(true)
        .with_position(PhysicalPosition::new(256, 512))
        .with_min_inner_size(LogicalSize::new(2048, 300))
        .with_max_inner_size(LogicalSize::new(2048, 300))
}
