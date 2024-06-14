// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::*;

// Next:
// - macos not working at all
// - 
// Cheatsheet:
// cargo tauri build -t x86_64-pc-windows-msvc
// 

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
        if drive_processes.len() != 7 { // win10
            app.get_window("main").unwrap().show().unwrap(); 
        }

        let count = drive_processes.len().to_string();
        let count = count.as_str();
        let local = chrono::offset::Local::now();
        let time = local.to_rfc2822();

        app.get_window("main").unwrap().set_title(
            format!("number_of_drive_processes: {count} --- {time}").as_str()
        );

        std::thread::sleep(std::time::Duration::from_millis(1000));
        // std::thread::sleep(std::time::Duration::from_millis(200));
            // Dev: 200, Rel: 1000

        if iteration.window_count == 0 { break; }
    }    
}
