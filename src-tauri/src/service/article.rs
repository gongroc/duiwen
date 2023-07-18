use std::ops::Deref;
use std::borrow::BorrowMut;
use chrono::Utc;
use rbatis::rbdc::datetime::FastDateTime;
use rust_i18n::t;
use crate::config::ctx::CtxManager;
use crate::config::error::Error;
use crate::event::{ARTICLE_CREATED, ARTICLE_DELETED, EVENT_EMITTER};
use crate::model::entity::{Article, ArticleFavorite, ArticleFolder};
use crate::model::repository::{delete_all_record, delete_sequece, insert_article, insert_article_favorite, insert_article_folder, insert_sequence, search_article_folder, select_all_article_favorite, select_all_article_folder, select_one_article, select_one_article_favorite, select_one_article_folder, update_article, update_article_favorite, update_article_folder, update_sequece};
use crate::service::{folder, sequence};
use crate::service::sequence::find_last_index;
use crate::utils::get_string_current_timestamp;

pub async fn create(title: String, folder_id: i64) -> Result<i64, Error> {
    folder::find_by_id(folder_id).await?;
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    let mut article_id = None;
    if let Some(id) = insert_article(&mut tx, title.as_str()).await?.last_insert_id.as_i64() {
        if let Some(af_id) = insert_article_folder(&mut tx, &folder_id, &id).await?.last_insert_id.as_i64() {
            let index = sequence::find_last_index(folder_id).await? + 1;
            insert_sequence(&mut tx, &folder_id, &af_id, "article",
                            &id, title.as_str(), &index).await?;
            article_id = Some(id);
            let mut event_emitter = EVENT_EMITTER.lock().unwrap();
            event_emitter.emit(ARTICLE_CREATED, id);
        }
    }
    tx.commit().await?;
    return if let Some(id) = article_id {
        Ok(id)
    } else {
        Err(Error::Business(t!("createArticleFailed")))
    };
}

pub async fn remove(id: i64) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    Article::delete_by_column(&mut tx, "id", id).await?;
    ArticleFolder::delete_by_column(&mut tx, "article_id", id).await?;
    delete_all_record(&mut tx, id.to_string().as_str(), "article").await?;
    delete_sequece(&mut tx, "", "", "article", id.to_string().as_str()).await?;
    ArticleFavorite::delete_by_column(&mut tx, "article_id", id).await?;
    tx.commit().await?;
    
    let mut event_emitter = EVENT_EMITTER.lock().unwrap();
    event_emitter.emit(ARTICLE_DELETED, id);
    
    Ok(())
}


pub async fn find_by_id(id: i64) -> Result<Article, Error> {
    let rb = CtxManager::get().sqlite.clone();
    if let Some(article) = select_one_article(&rb, &id).await? {
        return Ok(article);
    }
    Err(Error::System(t!("articleNotFound")))
}

pub async fn modify_title(id: i64, title: String) -> Result<(), Error> {
    if find_by_id(id).await?.title == title {
        return Ok(());
    }
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    update_article(&mut tx, title.as_str(), "", "", "", &id).await?;
    
    update_sequece(&mut tx, "", title.as_str(), "",
                   get_string_current_timestamp().as_str(), id.to_string().as_str(), "article", "").await?;
    update_article_favorite(&mut tx, &id, &FastDateTime::utc()).await?;
    tx.commit().await?;
    
    Ok(())
}

pub async fn modify_content(id: i64, content: String) -> Result<(), Error> {
    if content == find_by_id(id).await?.content {
        return Ok(());
    }
    let rb = CtxManager::get().sqlite.clone();
    let mut tx = rb.acquire_begin().await?;
    update_article(&mut tx, "", content.as_str(), "", "", &id).await?;
    update_article_favorite(&mut tx, &id, &FastDateTime::now()).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn modify_intro(id: i64, intro: String) -> Result<(), Error> {
    if intro == find_by_id(id).await?.intro {
        return Ok(());
    }
    let rb = CtxManager::get().sqlite.clone();
    update_article(rb.deref().borrow_mut(), "", "", intro.as_str(), "", &id).await?;
    Ok(())
}

pub async fn toggle_editing(id: i64, editing: bool) -> Result<(), Error> {
    if find_by_id(id).await?.editing == editing {
        return Ok(());
    }
    let rb = CtxManager::get().sqlite.clone();
    update_article(rb.deref().borrow_mut(), "", "", "", editing.to_string().as_str(), &id).await?;
    Ok(())
}

pub async fn link_folder(article_id: i64, folder_id: i64) -> Result<(), Error> {
    let article = find_by_id(article_id).await?;
    let rb = CtxManager::get().sqlite.clone();
    if find_article_folder(article_id, folder_id).await?.is_some() {
        return Ok(());
    }
    let mut tx = rb.acquire_begin().await?;
    if let Some(af_id) = insert_article_folder(&mut tx, &folder_id, &article_id).await?.last_insert_id.as_i64() {
        let index = find_last_index(folder_id).await? + 1;
        insert_sequence(&mut tx, &folder_id, &af_id, "article",
                        &article_id, article.title.as_str(), &index).await?;
    }
    tx.commit().await?;
    
    Ok(())
}

pub async fn unlink_folder(article_id: i64, folder_id: i64) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    // 文章最少属于一个文件夹
    let count = select_all_article_folder(rb.deref().borrow_mut(), "",
                                          article_id.to_string().as_str()).await?.len();
    if count == 1 {
        return Ok(());
    }
    
    if let Some(af) = find_article_folder(article_id, folder_id).await? {
        let mut tx = rb.acquire_begin().await?;
        ArticleFolder::delete_by_column(&mut tx, "id", af.id).await?;
        delete_sequece(&mut tx, "", af.id.to_string().as_str(), "article", "").await?;
        tx.commit().await?;
    }
    Ok(())
}

pub async fn find_article_folder(article_id: i64, folder_id: i64) -> Result<Option<ArticleFolder>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_one_article_folder(&rb, "", folder_id.to_string().as_str(),
                                 article_id.to_string().as_str()).await?)
}

pub async fn find_article_folders(article_id: i64) -> Result<Vec<ArticleFolder>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_all_article_folder(&rb, "", article_id.to_string().as_str()).await?)
}

pub async fn find_folder_articles(folder_id: i64) -> Result<Vec<ArticleFolder>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_all_article_folder(&rb, folder_id.to_string().as_str(), "").await?)
}


pub async fn search_folder_articles(folder_id: i64, keyword: String) -> Result<Vec<Article>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(search_article_folder(&rb, &folder_id, keyword.as_str()).await?)
}

pub async fn add_favorite(article_id: i64) -> Result<(), Error> {
    find_by_id(article_id).await?;
    if find_favorite(article_id).await?.is_some() {
        return Ok(());
    }
    let rb = CtxManager::get().sqlite.clone();
    insert_article_favorite(rb.deref().borrow_mut(), &article_id).await?;
    Ok(())
}

pub async fn find_favorite(article_id: i64) -> Result<Option<ArticleFavorite>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_one_article_favorite(&rb, "", article_id.to_string().as_str()).await?)
}

pub async fn remove_favorite(article_id: i64) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    ArticleFavorite::delete_by_column(rb.deref().borrow_mut(), "article_id", article_id).await?;
    Ok(())
}

pub async fn find_all_favorites() -> Result<Vec<ArticleFavorite>, Error> {
    let rb = CtxManager::get().sqlite.clone();
    Ok(select_all_article_favorite(&rb).await?)
}

pub async fn modify_folder_remark(id: i64, remark: String) -> Result<(), Error> {
    let rb = CtxManager::get().sqlite.clone();
    update_article_folder(rb.deref().borrow_mut(), &id, remark.as_str()).await?;
    Ok(())
}