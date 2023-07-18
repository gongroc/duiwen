pub mod setting;

use crate::config::error::Error;
use event_emitter_rs::EventEmitter;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub const LANG_SETTED: &str = "LANG_SETTED";
pub const WORKSPACE_SETTED: &str = "WORKSPACE_SETTED";
pub const DRAWER_CREATED: &str = "DRAWER_CREATED";
pub const DRAWER_CURRENT_CHANGED: &str = "DRAWER_CURRENT_CHANGED";
pub const ARTICLE_CREATED: &str = "ARTICLE_CREATED";
pub const ARTICLE_DELETED: &str = "ARTICLE_DELETED";
pub const FOLDER_CREATED: &str = "FOLDER_CREATED";
pub const FOLDER_DELETED: &str = "FOLDER_DELETED";

lazy_static! {
    pub static ref EVENT_EMITTER: Mutex<EventEmitter> = Mutex::new(EventEmitter::new());
}

/// 绑定事件
pub fn bind_event() -> Result<(), Error> {
    let mut event_emitter = EVENT_EMITTER.lock().unwrap();
    event_emitter.on(LANG_SETTED, setting::change_local_when_lang_setted);
    Ok(())
}
