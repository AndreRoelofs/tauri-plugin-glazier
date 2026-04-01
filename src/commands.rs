use crate::platform;
use tauri;

#[tauri::command]
pub async fn position_window_next_to_previous() {
    platform::commands::position_window_next_to_previous().await
}
