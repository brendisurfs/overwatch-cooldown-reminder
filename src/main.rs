pub mod audio;
pub mod events;
use events::handle_events;
use std::{cell::Cell, time::Duration};
use tokio::sync::watch::Receiver;

use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::PhysicalPosition, LogicalSize, WindowBuilder};

// use crate::audio::play_audio_idk;
use futures_util::StreamExt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CooldownMsg {
    HasCooldown,
    NoCooldown,
}

// keys tied to cooldowns.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CooldownKeys {
    ShiftKey,
    EKey,
}

pub struct AppProps {
    pub receiver: Cell<Option<Receiver<CooldownKeys>>>,
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::watch::channel(CooldownKeys::EKey);
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
    let e_key_status = use_state(cx, || CooldownMsg::HasCooldown);
    let shift_key_status = use_state(cx, || CooldownMsg::HasCooldown);

    // handle turn off e key cooldown
    use_effect(cx, e_key_status, |e_key_status| async move {
        if e_key_status.get() == &CooldownMsg::NoCooldown {
            tokio::time::sleep(Duration::from_secs(8)).await;
            e_key_status.set(CooldownMsg::HasCooldown);
            println!("reset cooldown.");
        }
    });
    let _: &Coroutine<()> = use_coroutine(cx, |_| {
        let recv = cx.props.receiver.take();
        let e_key_status = e_key_status.to_owned();
        let shift_key_status = shift_key_status.to_owned();
        async move {
            if let Some(mut r) = recv {
                while r.changed().await.is_ok() {
                    let msg = *r.borrow();
                    if msg == CooldownKeys::EKey {
                        if e_key_status.get() != &CooldownMsg::NoCooldown {
                            e_key_status.set(CooldownMsg::NoCooldown);
                            println!("EKEY Cooldown used.");
                        }
                    } else if msg == CooldownKeys::ShiftKey {
                        if shift_key_status.get() != &CooldownMsg::NoCooldown {
                            shift_key_status.set(CooldownMsg::NoCooldown);
                            println!("SHIFT Cooldown used.");
                        }
                    }
                }
            }
        }
    });

    // handle turn off shift key cooldown
    let shift_coroutine = use_coroutine(cx, |mut rx: UnboundedReceiver<CooldownMsg>| {
        let shift_key_status = shift_key_status.to_owned();
        async move {
            match rx.next().await {
                Some(msg) => match msg {
                    CooldownMsg::HasCooldown => println!("has cooldown msg"),
                    CooldownMsg::NoCooldown => {
                        tokio::time::sleep(Duration::from_secs(8)).await;
                        shift_key_status.set(CooldownMsg::HasCooldown);
                        println!("no cooldown msg");
                    }
                },
                None => (),
            }
        }
    });
    use_effect(cx, shift_key_status, |shift_key_status| {
        let shift_key_status = shift_key_status.to_owned();
        let shift_coroutine = shift_coroutine.to_owned();
        async move {
            if shift_key_status.get() == &CooldownMsg::NoCooldown {
                shift_coroutine.send(CooldownMsg::NoCooldown);
                println!("reset cooldown sent to coroutine");
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
            font_size: "60px",
            text_align: "center",
            background_color: "transparent",
            match e_key_status.get() {
                &CooldownMsg::HasCooldown => {
                    match shift_key_status.get() {
                        &CooldownMsg::HasCooldown => "USE YOUR COOLDOWNS",
                        &CooldownMsg::NoCooldown => "USE YOUR NADE",
                        }
                },
                &CooldownMsg::NoCooldown => {
                    match shift_key_status.get() {
                        &CooldownMsg::HasCooldown => "USE YOUR SLEEP",
                        &CooldownMsg::NoCooldown => "",
                    }
                }
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
