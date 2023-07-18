use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::ops::Deref;
use rust_i18n::t;
use async_recursion::async_recursion;
use rbatis::rbdc::datetime::FastDateTime;
use crate::config::ctx::CtxManager;
use crate::config::error::Error;
use crate::event::{EVENT_EMITTER, FOLDER_CREATED, FOLDER_DELETED};
use crate::model::entity::{Folder, FolderDrawer, FolderRelation};
use crate::model::repository::{delete_all_record, delete_article_folder, delete_sequece, insert_folder, insert_folder_drawer, insert_folder_relation, insert_sequence, select_all_folder_relations, select_drawer_top_level_folders, select_one_folder, select_one_folder_drawer, select_one_folder_relation, update_folder, update_folder_relation, update_sequece};
use crate::model::vo::{CountItem, FolderCascader, FolderMap, Link, Node};
use crate::service::{article, drawer, sequence};
use crate::utils::get_string_current_timestamp;


/// 修改文件夹标题
pub async fn modify_title(id: i64, title: String) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    let ts = get_string_current_timestamp();
    update_folder(&mut tx, title.as_str(), "", ts.as_str(), &id).await?;
    update_sequece(&mut tx, "", title.as_str(), id.to_string().as_str(),
                   ts.as_str(), "", "folder", "").await?;
    tx.commit().await?;
    
    Ok(())
}

/// 修改文件夹介绍
pub async fn modify_intro(id: i64, intro: String) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    let ts = get_string_current_timestamp();
    update_folder(rb.deref().borrow_mut(), "", intro.as_str(), ts.as_str(), &id).await?;
    Ok(())
}

/// 根据编号查询文件夹
pub async fn find_by_id(id: i64) -> Result<Folder, Error> {
    let rb = CtxManager::get().sqlite.clone();
    return if let Some(folder) = select_one_folder(&rb, &id).await? {
        Ok(folder)
    } else {
        Err(Error::Business(t!("folderNotFound")))
    };
}

/// 新建子文件夹，并与父文件夹建立关联
pub async fn create_subset(title: String, intro: String, start_id: i64, link_title: String) -> Result<(), Error> {
    find_by_id(start_id).await?;
    
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    
    // 查询父文件夹所在组
    let start_drawer = find_folder_drawer(start_id).await?;
    let index = sequence::find_last_index(start_id).await? + 1;
    
    // 新建子文件夹
    if let Some(end_id) = insert_folder(&mut tx, title.as_str(), intro.as_str()).await?.last_insert_id.as_i64() {
        // 设置子文件夹组
        insert_folder_drawer(&mut tx, &end_id, &start_drawer.drawer_id).await?;
        // 关联两个文件夹
        if let Some(fr_id) = insert_folder_relation(&mut tx, start_id.to_string().as_str(),
                                                    &end_id, link_title.as_str()).await?.last_insert_id.as_i64() {
            insert_sequence(&mut tx, &start_id, &fr_id, "folder", &end_id,
                            title.as_str(), &index).await?;
        }
        
        let mut event_emitter = EVENT_EMITTER.lock().unwrap();
        event_emitter.emit(FOLDER_CREATED, end_id);
    }
    
    tx.commit().await?;
    
    Ok(())
}

/// 链接两个已有文件夹
pub async fn link(start_id: i64, end_id: i64, title: String) -> Result<(), Error> {
    // 检查文件夹是否存在
    find_by_id(start_id).await?;
    find_by_id(end_id).await?;
    
    let rb = CtxManager::get().sqlite.clone();
    
    // 检查是否已关联
    if find_relation(start_id, end_id).await?.is_some() {
        return Ok(());
    }
    
    // 检查两个文件夹是否属于同一个组
    if find_folder_drawer(start_id).await?.drawer_id != find_folder_drawer(end_id).await?.drawer_id {
        return Err(Error::Business(t!("folderIsNotInTheSameGroup")));
    }
    let mut tx = rb.acquire_begin().await?;
    
    if let Some(fr_id) = insert_folder_relation(&mut tx, start_id.to_string().as_str(),
                                                &end_id, title.as_str()).await?.last_insert_id.as_i64() {
        let index = sequence::find_last_index(start_id).await? + 1;
        let folder = find_by_id(end_id).await?;
        insert_sequence(&mut tx, &start_id, &fr_id, "folder",
                        &end_id, folder.title.as_str(), &index).await?;
    }
    tx.commit().await?;
    Ok(())
}

/// 取消文件夹关联
/// TODO 取消关联之后，暂时先处理为将当前文件夹设为顶级
pub async fn unlink(relation_id: i64) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    
    if let Ok(fr) = find_relation_by_id(relation_id).await {
        let mut tx = rb.acquire_begin().await?;
        // FolderRelation::delete_by_column(&mut tx, "id", relation_id).await?;
        update_folder_relation(&mut tx, &relation_id, "NULL", "", "").await?;
        delete_sequece(&mut tx, "", fr.id.to_string().as_str(), "folder", "").await?;
        tx.commit().await?;
    }
    
    Ok(())
}

/// 设置为顶级文件夹，即无父关系
pub async fn set_as_top_level(relation_id: i64) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    update_folder_relation(&mut tx, &relation_id, "NULL", "", "").await?;
    delete_sequece(&mut tx, "", relation_id.to_string().as_str(), "folder", "").await?;
    tx.commit().await?;
    Ok(())
}


/// 创建顶级文件夹
pub async fn create_top_level(drawer_id: i64, title: String, intro: String) -> Result<(), Error> {
    drawer::find_by_id(drawer_id).await?;
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    
    if let Some(folder_id) = insert_folder(&mut tx, title.as_str(), intro.as_str()).await?.last_insert_id.as_i64() {
        insert_folder_drawer(&mut tx, &folder_id, &drawer_id).await?;
        insert_folder_relation(&mut tx, "", &folder_id, "").await?;
        
        let mut event_emitter = EVENT_EMITTER.lock().unwrap();
        event_emitter.emit(FOLDER_CREATED, folder_id);
    }
    
    tx.commit().await?;
    
    Ok(())
}

/// 查询文件夹所属的组
pub async fn find_folder_drawer(folder_id: i64) -> Result<FolderDrawer, Error> {
    let rb = CtxManager::get().sqlite.clone();
    return if let Some(folder_drawer) = select_one_folder_drawer(&rb, "",
                                                                 folder_id.to_string().as_str()).await? {
        Ok(folder_drawer)
    } else {
        Err(Error::Business(t!("theGroupOfTheFolderWasNotFound")))
    };
}

/// 查询无关系文件夹，即顶级文件夹
pub async fn find_all_top_level(drawer_id: i64) -> Result<Vec<FolderRelation>, Error> {
    drawer::find_by_id(drawer_id).await?;
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_drawer_top_level_folders(&rb, &drawer_id).await?)
}

/// 根据文件夹查询相关联的文件夹列表，子集或父级
/// is_end: true 查询 end_id , true 查询 start_id
pub async fn find_all_relations(folder_id: i64, is_end: bool) -> Result<Vec<FolderRelation>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    let mut items = vec![];
    if is_end {
        for folder_relation in select_all_folder_relations(&rb, "", folder_id.to_string().as_str()).await? {
            items.push(folder_relation);
        }
    } else {
        for folder_relation in select_all_folder_relations(&rb, folder_id.to_string().as_str(), "").await? {
            items.push(folder_relation);
        }
    }
    Ok(items)
}

/// 根据编号查询文件夹关系
pub async fn find_relation_by_id(id: i64) -> Result<FolderRelation, Error> {
    let rb = CtxManager::get().sqlite.clone();
    if let Some(relation) = select_one_folder_relation(&rb, "", "", id.to_string().as_str()).await? {
        return Ok(relation);
    }
    Err(Error::Business(t!("folderNotAssociated")))
}

/// 查询文件夹之间关联
pub async fn find_relation(start_id: i64, end_id: i64) -> Result<Option<FolderRelation>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_one_folder_relation(&rb, start_id.to_string().as_str(), end_id.to_string().as_str(), "").await?)
}

///  修改关系名
pub async fn modify_relation_title(title: String, relation_id: i64) -> Result<(), Error> {
    find_relation_by_id(relation_id).await?;
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    update_folder_relation(&mut tx, &relation_id, "", "", title.as_str()).await?;
    update_sequece(&mut tx, "", "", "", FastDateTime::now().to_string().as_str(),
                   "", "folder", relation_id.to_string().as_str()).await?;
    tx.commit().await?;
    Ok(())
}

/// 查询节点关系图
pub async fn find_relation_map(root_id: Option<i64>, up_deep: Option<i32>, down_deep: Option<i32>) -> Result<FolderMap, Error> {
    let mut nodes = HashSet::new();
    let mut links = vec![];
    let mut up_index = 0;
    let mut down_index = 0;
    let mut next_level_ids = vec![];
    let mut pre_level_ids = vec![];
    
    // 组织起点
    if let Some(root_id) = root_id {
        let root_folder = find_by_id(root_id).await?;
        nodes.insert(Node { id: root_id, title: root_folder.title });
        pre_level_ids.push(root_id);
    } else {
        let current_drawer = drawer::find_current().await?;
        // 将文件夹设为顶级节点
        nodes.insert(Node { id: -1, title: current_drawer.title });
        
        // 查询顶级文件夹
        for fr in find_all_top_level(current_drawer.id).await? {
            next_level_ids.push(fr.end_id);
            let folder = find_by_id(fr.end_id).await?;
            nodes.insert(Node { id: folder.id, title: folder.title });
            links.push(Link {
                source: folder.id,
                target: -1,
                label: "".into(),
            });
        }
    }
    
    // 向下查询
    loop {
        if let Some(down) = down_deep {
            if down == down_index {
                break;
            }
        }
        if next_level_ids.is_empty() {
            break;
        }
        let mut tmp_next_level_ids = vec![];
        for next_level_id in &next_level_ids {
            for fr in find_all_relations(*next_level_id, false).await? {
                let folder = find_by_id(fr.end_id).await?;
                nodes.insert(Node { id: folder.id, title: folder.title });
                links.push(Link {
                    source: fr.start_id.unwrap_or(-1),
                    target: fr.end_id,
                    label: fr.title.clone(),
                });
                tmp_next_level_ids.push(fr.end_id);
            }
        }
        next_level_ids = tmp_next_level_ids;
        down_index += 1;
    }
    
    // 向上查询
    loop {
        if let Some(up) = up_deep {
            if up == up_index {
                break;
            }
        }
        if pre_level_ids.is_empty() {
            break;
        }
        let mut tmp_pre_level_ids = vec![];
        
        for pre_level_id in &pre_level_ids {
            for fr in find_all_relations(*pre_level_id, true).await? {
                if let Some(start_id) = fr.start_id {
                    let folder = find_by_id(start_id).await?;
                    nodes.insert(Node { id: folder.id, title: folder.title });
                    links.push(Link {
                        source: start_id,
                        target: fr.end_id,
                        label: fr.title,
                    });
                    tmp_pre_level_ids.push(start_id);
                }
            }
        }
        
        pre_level_ids = tmp_pre_level_ids;
        up_index += 1;
    }
    
    Ok(FolderMap { nodes, links })
}

/// 删除文件夹
pub async fn remove(id: i64) -> Result<(), Error> {
    if find_by_id(id).await.is_err() {
        return Ok(());
    }
    
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    
    for fr in find_all_relations(id, false).await? {
        update_folder_relation(&mut tx, &fr.id, "NULL", "", "").await?;
    }
    
    Folder::delete_by_column(&mut tx, "id", id).await?;
    FolderDrawer::delete_by_column(&mut tx, "folder_id", id).await?;
    FolderRelation::delete_by_column(&mut tx, "end_id", id).await?;
    delete_all_record(&mut tx, id.to_string().as_str(), "folder").await?;
    // delete_all_favorites(&mut tx, id.to_string().as_str(), "folder").await?;
    delete_sequece(&mut tx, "", "", "folder", id.to_string().as_str()).await?;
    delete_article_folder(&mut tx, id.to_string().as_str(), "").await?;
    
    tx.commit().await?;
    
    let mut event_emitter = EVENT_EMITTER.lock().unwrap();
    event_emitter.emit(FOLDER_DELETED, id);
    
    Ok(())
}

pub async fn find_folder_cascader(drawer_id: i64) -> Result<Vec<FolderCascader>, Error> {
    let mut items = vec![];
    for fr in find_all_top_level(drawer_id).await? {
        items.push(find_cascder(fr.end_id).await?);
    }
    Ok(items)
}

#[async_recursion]
pub async fn find_cascder(folder_id: i64) -> Result<FolderCascader, Error> {
    let folder = find_by_id(folder_id).await?;
    let subfolders = find_all_relations(folder_id, false).await?;
    let mut children = vec![];
    for fr in subfolders {
        children.push(find_cascder(fr.end_id).await?);
    }
    
    Ok(FolderCascader {
        value: folder.id,
        label: folder.title,
        children,
    })
}


pub async fn count(folder_id: i64) -> Result<CountItem, Error> {
    Ok(CountItem {
        article: article::find_folder_articles(folder_id).await?.len(),
        folder: find_all_relations(folder_id, false).await?.len(),
    })
}
