use std::borrow::BorrowMut;
use std::ops::Deref;
use rust_i18n::t;
use crate::config::ctx::CtxManager;
use crate::config::error::Error;
use crate::event::{DRAWER_CREATED, DRAWER_CURRENT_CHANGED, EVENT_EMITTER};
use crate::model::entity::{Drawer};
use crate::model::repository::{insert_drawer, search_drawer, select_drawer_max_index, select_one_drawer, update_drawer};

/// 创建组
pub async fn create(title: String) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    
    let max_index = select_drawer_max_index(&rb).await?.unwrap_or(-1) + 1;
    
    if let Some(id) = insert_drawer(rb.deref().borrow_mut(), title.as_str(), &max_index).await?.last_insert_id.as_i64() {
        let mut event_emitter = EVENT_EMITTER.lock().unwrap();
        event_emitter.emit(DRAWER_CREATED, id);
    }
    
    Ok(())
}

/// 根据关键词搜索组
pub async fn search(keyword: String) -> Vec<Drawer> {
    let rb = CtxManager::get().sqlite.clone();
    
    let mut items = vec![];
    
    if let Ok(groups) = search_drawer(&rb, keyword.as_str()).await {
        items = groups;
    }
    
    return items;
}

/// 修改组名称
pub async fn modify_title(title: String, id: i64) -> Result<(), Error> {
    if find_by_id(id).await?.title == title {
        return Ok(());
    }
    let rb = CtxManager::get().sqlite.clone();
    update_drawer(rb.deref().borrow_mut(), title.as_str(), "", "", "", &id).await?;
    Ok(())
}

/// 修改组介绍
pub async fn modify_intro(intro: String, id: i64) -> Result<(), Error> {
    if let Some(val) = find_by_id(id).await?.intro {
        if val == intro {
            return Ok(());
        }
    }
    
    let rb = CtxManager::get().sqlite.clone();
    update_drawer(rb.deref().borrow_mut(), "", intro.as_str(), "", "", &id).await?;
    Ok(())
}

/// 根据编号查找组
pub async fn find_by_id(id: i64) -> Result<Drawer, Error> {
    let rb = CtxManager::get().sqlite.clone();
    if let Some(drawer) = select_one_drawer(&rb, id.to_string().as_str(), "").await? {
        return Ok(drawer);
    }
    Err(Error::Business(t!("groupNotFound")))
}

/// 更换当前组，并且将当前组序列排最高位置
pub async fn change_current(id: i64) -> Result<(), Error> {
    let drawer = find_by_id(id).await?;
    if drawer.current {
        return Ok(());
    }
    
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    
    let mut index = 0;
    
    for item in Drawer::select_all(&mut tx).await? {
        if index < item.idx {
            index = item.idx;
        }
        update_drawer(&mut tx, "", "", "", "false", &item.id).await?;
    }
    
    let idx = index + 1;
    update_drawer(&mut tx, "", "", idx.to_string().as_str(), "true", &id).await?;
    tx.commit().await?;
    
    let mut event_emitter = EVENT_EMITTER.lock().unwrap();
    event_emitter.emit(DRAWER_CURRENT_CHANGED, id);
    
    Ok(())
}

/// 查找当前组
pub async fn find_current() -> Result<Drawer, Error> {
    let rb = CtxManager::get().sqlite.clone();
    if let Some(drawer) = select_one_drawer(&rb, "", "true").await? {
        return Ok(drawer);
    }
    // TODO 默认组未找到的话，随便指定一个
    Err(Error::Business(t!("defaultGroupNotFound")))
}
