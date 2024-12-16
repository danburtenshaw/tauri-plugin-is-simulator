use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<IsSimulator<R>> {
  Ok(IsSimulator(app.clone()))
}

/// Access to the is-simulator APIs.
pub struct IsSimulator<R: Runtime>(AppHandle<R>);

impl<R: Runtime> IsSimulator<R> {
  pub fn is_simulator(&self) -> crate::Result<IsSimulatorResponse> {
    Ok(IsSimulatorResponse {
      value: false,
    })
  }
}
