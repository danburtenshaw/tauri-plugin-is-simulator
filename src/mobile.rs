use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_is_simulator);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<IsSimulator<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.issimulator", "IsSimulatorPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_is_simulator)?;

    Ok(IsSimulator(handle))
}

/// Access to the is-simulator APIs.
pub struct IsSimulator<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> IsSimulator<R> {
    pub fn is_simulator(&self) -> crate::Result<IsSimulatorResponse> {
        self.0
            .run_mobile_plugin("isSimulator", None::<String>)
            .map_err(Into::into)
    }
}
