use crate::config::error::Error;
use crate::event;
use crate::event::EVENT_EMITTER;
use crate::system::dir::get_base_dir_path;
use rust_i18n::t;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

fn get_cfg_file() -> Result<File, Error> {
    let file_path = get_base_dir_path()?.join("cfg.json");
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(false)
        .create_new(!file_path.exists())
        .open(file_path)?;
    Ok(file)
}

/// 设置配置文件内容
fn set_cfg(key: &str, value: &str) -> Result<(), Error> {
    let mut cfg = get_cfg()?;
    cfg.insert(String::from(key), String::from(value));
    let mut file = get_cfg_file()?;
    let json_str = serde_json::to_string(&cfg)?;
    file.write_all(json_str.as_bytes())?;
    file.flush()?;
    Ok(())
}

/// 获得配置文件所有内容
fn get_cfg() -> Result<HashMap<String, String>, Error> {
    let mut file = get_cfg_file()?;

    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    return if json_str.is_empty() {
        Ok(HashMap::new())
    } else {
        Ok(serde_json::from_str::<HashMap<String, String>>(
            json_str.as_str(),
        )?)
    };
}

/// 获得当前语言环境
pub fn get_lang() -> Result<Option<String>, Error> {
    let cfg = get_cfg()?;
    Ok(cfg.get("lang").cloned())
}

/// 设置语言环境
/// 只支持中文与英文
/// 将语言环境保存到配置文件中，并更新当前程序环境
pub fn set_lang(value: String) -> Result<(), Error> {
    if ![String::from("zh-CN"), String::from("en-US")].contains(&value) {
        return Err(Error::System(t!("wrongLanguageFormat")));
    }
    if let Some(lang) = get_lang()? {
        if lang == value {
            return Ok(());
        }
    }
    set_cfg("lang", value.as_str())?;
    EVENT_EMITTER
        .lock()
        .unwrap()
        .emit(event::LANG_SETTED, value);
    Ok(())
}

/// 设置工作空间
/// 设置成功后发送事件
pub fn set_workspace(value: String) -> Result<(), Error> {
    if !Path::is_dir(value.as_ref()) {
        return Err(Error::System(t!("wrongFolderFormat")));
    }
    if let Some(workspace) = get_workspace()? {
        if workspace == value {
            return Ok(());
        }
    }
    // 去掉路径最后的 / 分隔符
    let mut workspace_path = value.clone();
    loop {
        if !workspace_path.ends_with("/") {
            break;
        }
        workspace_path = workspace_path[0..workspace_path.len() - 1].to_string();
    }
    set_cfg("workspace", workspace_path.as_str())?;
    EVENT_EMITTER
        .lock()
        .unwrap()
        .emit(event::WORKSPACE_SETTED, value);
    Ok(())
}

/// 获得当前工作空间，如果没有的话则None
pub fn get_workspace() -> Result<Option<String>, Error> {
    let cfg = get_cfg()?;
    Ok(cfg.get("workspace").cloned())
}
