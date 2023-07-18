pub mod cfg;
pub mod dir;
pub mod init;

use crate::config::error::Error;
use crate::event;
use tauri::{Window, Wry};

/// 初始化基础服务
pub fn init() {
    if let Err(err) = init_steps() {
        use std::thread::sleep;
        use std::time::Duration;
        use tauri::api::notification::Notification;
        Notification::new("com.duiwen.editor")
            .title("对文笔记核心服务启动失败")
            .body(err.to_string())
            .show()
            .unwrap();
        sleep(Duration::from_millis(500));
        std::process::exit(1);
    }
}

/// 核心系统服务初始化
fn init_steps() -> Result<(), Error> {
    init::log()?;
    init::i18n()?;
    event::bind_event()?;
    Ok(())
}

/// 设置窗口效果
pub fn set_window_style(window: &Window<Wry>) {
    window_shadows::set_shadow(window, true).expect("Unsupported platform!");
}
