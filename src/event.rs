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
