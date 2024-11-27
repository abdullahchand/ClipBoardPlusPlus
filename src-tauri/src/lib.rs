mod clipboard_functions;
mod tray;
mod controller;
mod fileio;
mod utils;
use tauri::{AppHandle, Emitter, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn write_to_clipboard_call(content: &str) -> Result<(), String>{
    crate::clipboard_functions::write_clipboard(content).unwrap();
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![write_to_clipboard_call,
            crate::clipboard_functions::monitor_clipboard_changes,
            crate::controller::login_with_google,
            crate::controller::get_user,
            crate::fileio::logout_user])
        .setup(|app| {
            #[cfg(target_os = "linux")]{
                tray::create_linux_tray(app.handle())?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
