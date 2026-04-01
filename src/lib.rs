use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod error;

pub use error::{Error, Result};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("glazier")
        .setup(|_app, _api| {
            log::info!("glazier plugin initialized");
            Ok(())
        })
        .build()
}
