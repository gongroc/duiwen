use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Mutex};
use tauri::{Window, Wry};

lazy_static! {
    static ref MESSAGES: Mutex<Vec<Message>> = Mutex::new(vec![]);
}

/// 消息结构
#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub payload: Option<String>,
    pub event: MessageEvent,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum MessageEvent {
    // 弹出消息
    AlertMessage,
    // 状态栏消息
    StatusBarMessage,
    // 跳转页面
    JumpPage,
}

/// 发布站内消息
pub fn push(event: MessageEvent, payload: Option<String>) {
    let message = Message {
        payload,
        event,
        timestamp: Utc::now(),
    };
    if let Ok(mut messages) = MESSAGES.lock() {
        messages.push(message);
    } else {
        log::error!("发布系统消息失败,{:?}", &message)
    }
}

/// 转发消息到窗口
pub fn forword(window: &Window<Wry>) {
    if let Some(message) = MESSAGES.lock().unwrap().pop() {
        if let Err(err) = window.emit("message_hub", json!(message)) {
            log::error!("通知窗口消息失败:{:?}", err)
        }
    }
}
