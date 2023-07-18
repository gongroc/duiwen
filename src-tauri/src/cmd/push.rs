use crate::config::message;
use tauri::{Window, Wry};

#[tauri::command]
pub fn message_hub(window: Window<Wry>) {
    use std::thread::sleep;
    use std::time::Duration;

    tauri::async_runtime::spawn(async move {
        loop {
            sleep(Duration::from_secs(1));
            message::forword(&window);
        }
    });
}
