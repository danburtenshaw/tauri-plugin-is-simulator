use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::IsSimulatorExt;
use crate::Result;

#[command]
pub(crate) async fn is_simulator<R: Runtime>(app: AppHandle<R>) -> Result<IsSimulatorResponse> {
    app.is_simulator().is_simulator()
}
