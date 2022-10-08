#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
mod cmds;
mod event;
mod menu;
mod tray;

fn main() {
    tauri::Builder::default()
        .menu(menu::main_menu_builder())
        .setup(|app| {
            app.get_window("main").and_then(|win| {
                let pkg_info = app.package_info();
                let window_title = format!("{} - v{}", pkg_info.name, pkg_info.version);
                win.set_title(window_title.as_str()).ok()
            });
            event::register_global_events(app);
            event::register_alarm_events(app);
            Ok(())
        })
        .system_tray(tray::SystemTrayBuilder::build())
        .on_system_tray_event(tray::SystemTrayBuilder::handle_tray_event)
        .invoke_handler(tauri::generate_handler![cmds::test])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(event::handle_run_events);
}
