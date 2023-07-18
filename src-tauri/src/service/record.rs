use std::ops::Deref;
use std::borrow::BorrowMut;
use crate::config::ctx::CtxManager;
use crate::config::error::Error;
use crate::model::entity::Record;
use crate::model::repository::{delete_record, insert_record, select_all_record, select_last_record};
use crate::service::{article, folder};

/// 创建一条记录
async fn create(target_id: i64, target_type: &str) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    
    if let Some(record) = select_last_record(&rb).await? {
        if record.target_id == target_id && record.target_type == target_type {
            return Ok(());
        }
    }
    
    insert_record(rb.deref().borrow_mut(), &target_id, target_type).await?;
    Ok(())
}

/// 记录文章被打开
pub async fn push_article(artice_id: i64) -> Result<(), Error> {
    article::find_by_id(artice_id).await?;
    create(artice_id, "article").await?;
    Ok(())
}

/// 记录文件夹被打开
pub async fn push_folder(folder_id: i64) -> Result<(), Error> {
    folder::find_by_id(folder_id).await?;
    create(folder_id, "folder").await?;
    Ok(())
}

/// 保留一定数数据的前提下，删除旧的记录
pub async fn remove_old_rows(keep_rows: i32) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    delete_record(rb.deref().borrow_mut(), &keep_rows).await?;
    Ok(())
}

/// 删除全部记录
pub async fn remove_all() -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    delete_record(rb.deref().borrow_mut(), &0).await?;
    Ok(())
}

/// 查询全部记录
pub async fn find_all() -> Result<Vec<Record>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_all_record(&rb, "", "").await?)
}








