// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::*;

// Next:
// - dev - sys tray quit
// - check if drive running
//   . Hide/unhide main win in a loop https://docs.rs/tauri/latest/tauri/struct.App.html#method.run_iteration
//   . /Users/gsimsek/code/rust/systats-rs/src-tauri/src/monitors/cpu.rs
// - show error window when drive not running
// - 

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    let tray = SystemTray::new().with_menu(tray_menu);

    // On close, hide to try, work in the background
    let mut app = tauri::Builder::default()
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
                  "quit" => { std::process::exit(0); }
                  _ => {}
                }
            }
            _ => {}
		})
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    loop {
        let iteration = app.run_iteration();

        let process = "GoogleDriveFS";
        let sys = sysinfo::System::new_all();
        let drive_processes: Vec<&sysinfo::Process> 
            = sys.processes_by_name(&process).collect();
        if drive_processes.len() == 0 { // todo: 6 ?
            app.get_window("main").unwrap().show().unwrap(); 
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));

        if iteration.window_count == 0 { break; }
    }    
}
