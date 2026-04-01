use std::sync::Mutex;

use focus_tracker::FocusedWindow;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Emitter, Error, Listener, Runtime, WebviewWindow,
};

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

mod commands;

pub struct GlazierState {
    pub items: Mutex<Vec<FocusedWindow>>,
}

pub trait WebviewWindowExt {
    fn create_overlay_titlebar(&self) -> Result<&Self, Error>;
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("glazier")
        .setup(|app, _api| {
            app.manage(GlazierState {
                items: Mutex::new(Vec::new()),
            });
            log::info!("glazier plugin initialized");
            Ok(())
        })
        .build()
}
