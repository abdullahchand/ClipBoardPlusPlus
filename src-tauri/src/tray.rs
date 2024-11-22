use std::process;

use tauri::{tray::TrayIconBuilder, Runtime, menu::{Menu, MenuItem},
    Manager,
    tray::{MouseButton, MouseButtonState,TrayIconEvent}};
use tauri_plugin_positioner::{WindowExt, Position};



pub fn create_linux_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()>{
    let show_ui = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_ui])?;

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(|app, event| {

            match event.id.as_ref() {

            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            "open" => {
                println!("Open menu item was clicked");
                let tray_app = app.app_handle();
                if let Some(window) = tray_app.get_webview_window("main") {
                    if !window.is_visible().unwrap_or(false){
                        let _ = window.show();
                        let _ = window.set_focus();
                        window.set_decorations(true).unwrap();

                    }else {
                        window.hide().unwrap();
                    }

                }

            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        }})
        .build(app)?;
    Ok(())
}


pub fn create_mac_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()>{
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_ui = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_ui, &quit_i])?;

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
            let tray_app = tray.app_handle();
            tauri_plugin_positioner::on_tray_event(tray_app, &event);
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
            ..
            } => {
                println!("left click pressed and released");
                // in this example, let's show and focus the main window when the tray is clicked

                if let Some(window) = tray_app.get_webview_window("main") {
                    if !window.is_visible().unwrap_or(false){
                        let _ = window.move_window(tauri_plugin_positioner::Position::TrayBottomCenter);
                        let _ = window.show();
                        let _ = window.set_focus();

                    }else {
                        window.hide().unwrap();
                    }

                }
            }
            _ => {
                println!("unhandled event {event:?}");
            }
        }})
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            "open" => {
                println!("Open menu item was clicked");
                let tray_app = app.app_handle();
                if let Some(window) = tray_app.get_webview_window("main") {
                    if !window.is_visible().unwrap_or(false){
                        let _ = window.move_window(tauri_plugin_positioner::Position::TrayBottomCenter);
                        let _ = window.show();
                        let _ = window.set_focus();

                    }else {
                        window.hide().unwrap();
                    }

                }

            }
                _ => {
                    println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;
    Ok(())
}
