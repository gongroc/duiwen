use crate::config::error::Error;
use rust_i18n::t;
use std::path::PathBuf;
use tauri::api::path::home_dir;
use std::fs::create_dir_all;

/// 获得程序根文件夹路径
pub fn get_base_dir_path() -> Result<PathBuf, Error> {
    if let Some(home_dir) = home_dir() {
        let base_path = home_dir.join(".duiwen");
        if !&base_path.exists() {
            if let Err(err) = create_dir_all(&base_path) {
                log::error!("创建配置文件文件夹失败：{:?}",err);
                return Err(Error::System(t!("createConfigFolderFailed")));
            }
        }
        return Ok(base_path);
    }
    Err(Error::System(t!("unableToAccessConfigDirectory")))
}
