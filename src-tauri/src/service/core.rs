use std::path::Path;
use rust_i18n::t;
use crate::config::ctx::CtxManager;
use crate::config::error::Error;
use crate::model::repository::{count_all_drawer, insert_drawer, update_drawer};
use crate::service::record;
use crate::system::cfg;


// 初始化核心服务
pub async fn init() -> Result<(), Error> {
    let workspace = check_folder()?;
    init_ctx(workspace).await?;
    insert_default_data().await?;
    clear_old_data().await?;
    init_tray()?;
    Ok(())
}

fn create_dir(folder_path: String) -> Result<(), Error> {
    let workspace_path = Path::new(folder_path.as_str());
    if !workspace_path.exists() {
        if let Err(err) = std::fs::create_dir_all(workspace_path) {
            log::error!("check_folder:{}",err);
            return Err(Error::System(t!("createWorkspaceFolderFailed")));
        }
    }
    Ok(())
}

fn check_folder() -> Result<String, Error> {
    return if let Some(workspace) = cfg::get_workspace()? {
        create_dir(format!("{workspace}/article"))?;
        create_dir(format!("{workspace}/folder"))?;
        Ok(workspace)
    } else {
        Err(Error::System(t!("workspaceFolderNotSet")))
    };
}

pub async fn init_ctx(workspace: String) -> Result<(), Error> {
    CtxManager::init(workspace).await
}

pub async fn insert_default_data() -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    if count_all_drawer(&rb).await? == 0 {
        let mut tx = rb.acquire_begin().await?;
        if let Some(id) = insert_drawer(&mut tx, "duiwen", &0).await?.last_insert_id.as_i64() {
            update_drawer(&mut tx, "", "", "", "true", &id).await?;
        }
        tx.commit().await?;
    }
    Ok(())
}

pub async fn clear_old_data() -> Result<(), Error> {
    record::remove_old_rows(100).await?;
    Ok(())
}

pub fn init_tray() -> Result<(), Error> {
    Ok(())
}

