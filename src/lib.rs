use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::IsSimulator;
#[cfg(mobile)]
use mobile::IsSimulator;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the is-simulator APIs.
pub trait IsSimulatorExt<R: Runtime> {
  fn is_simulator(&self) -> &IsSimulator<R>;
}

impl<R: Runtime, T: Manager<R>> crate::IsSimulatorExt<R> for T {
  fn is_simulator(&self) -> &IsSimulator<R> {
    self.state::<IsSimulator<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("is-simulator")
    .invoke_handler(tauri::generate_handler![commands::is_simulator])
    .setup(|app, api| {
      #[cfg(mobile)]
      let is_simulator = mobile::init(app, api)?;
      #[cfg(desktop)]
      let is_simulator = desktop::init(app, api)?;
      app.manage(is_simulator);
      Ok(())
    })
    .build()
}
