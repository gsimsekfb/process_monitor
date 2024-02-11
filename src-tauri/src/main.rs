// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::*;

// Next:
// - dev - sys tray quit
// - check if drive running
// - show error window when drive not running
// - 

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
    let tray = SystemTray::new().with_menu(tray_menu);

    // On close, hide to try, work in the background
    tauri::Builder::default()
        .system_tray(tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        // Show/hide main window when tray icon left clicked
		.on_system_tray_event(|app, event| match event {
			SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
				let window = app.get_window("main").unwrap();
				if window.is_visible().unwrap() {
					window.hide().unwrap();
				} else {
					window.show().unwrap();
					window.set_focus().unwrap();
				}
			},
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                  "quit" => {
                    std::process::exit(0);
                  }
                  _ => {}
                }
            }            
            _ => {}
		})
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
