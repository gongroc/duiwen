use std::borrow::BorrowMut;
use std::ops::Deref;
use crate::config::ctx::CtxManager;
use crate::config::error::Error;
use crate::model::entity::Sequence;
use crate::model::repository::{search_all_sequece, select_last_sequece, update_sequece};
use crate::service::folder;

pub async fn search(root_folder: i64, relation_type: Option<String>, sort_by_idx: bool) -> Result<Vec<Sequence>, Error> {
    folder::find_by_id(root_folder).await?;
    let rb = CtxManager::get().sqlite.clone();
    Ok(search_all_sequece(&rb, "", &root_folder,
                          relation_type.unwrap_or(String::new()).as_str(), &sort_by_idx).await?)
}

pub async fn find_last_index(folder_id: i64) -> Result<i32, Error> {
    let rb = CtxManager::get().sqlite.clone();
    if let Some(s) = select_last_sequece(&rb, &folder_id).await? {
        return Ok(s.idx);
    }
    Ok(0)
}

pub async fn modify_index(id: i64, index: i32) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    
    update_sequece(rb.deref().borrow_mut(), id.to_string().as_str(), "",
                   index.to_string().as_str(), "", "", "", "").await?;
    
    Ok(())
}
