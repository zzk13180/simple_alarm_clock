use tauri::{App, AppHandle, Manager, RunEvent, Wry};
pub fn handle_run_events(app_handle: &AppHandle<Wry>, e: RunEvent) {
    match e {
        RunEvent::Exit => {}
        RunEvent::ExitRequested { .. } => {}
        RunEvent::WindowEvent {
            label: _, event, ..
        } => match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = app_handle.get_window("main").map(|win| {
                    let _ = win.hide();
                });
            }
            _ => {}
        },
        RunEvent::Ready => {}
        RunEvent::Resumed => {}
        RunEvent::MainEventsCleared => {}
        _ => {}
    }
}

pub fn register_global_events(app: &App) {
    app.listen_global("event_name", |event| {
        println!("{:?}", event);
    });
}

pub fn register_alarm_events(app: &App) {
    let window = app.get_window("main").unwrap();
    tauri::async_runtime::spawn(async move {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
            window.show().unwrap();
            window.set_focus().unwrap();
            println!("提醒");
        }
    });
}
