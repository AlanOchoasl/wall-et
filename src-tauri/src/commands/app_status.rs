use tauri::State;

use crate::AppState;
use crate::application::app_status::{self, AppStatusDto};
use crate::application::error::AppError;

#[tauri::command]
pub fn get_app_status(state: State<'_, AppState>) -> Result<AppStatusDto, AppError> {
    app_status::get(&state.database_path).map_err(AppError::from)
}
