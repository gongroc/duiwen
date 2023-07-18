use rbatis::rbdc::datetime::FastDateTime;
use crate::config::error::Error;
use crate::{system, service};
use tauri::{Window, Wry};
use crate::model::entity::{Article, ArticleFavorite, Drawer, Folder, FolderRelation};
use crate::model::vo::{ArticleItem, ArticleLinkedFolderItem, CountItem, FavoriteItem, FolderCascader, FolderItem, FolderMap,
                       FolderRelationItem, RecordItem, ResultItem};
use crate::service::{article, folder, sequence};

#[tauri::command]
pub async fn test() {
    // message::push(MessageEvent::AlertMessage, Some(content));
    // event::EVENT_EMITTER.lock().unwrap().emit("test", content);
    
    // let mut rb = CtxManager::get().sqlite.clone();
    // folder::test_insert(&mut rb, "123").await.expect("TODO: panic message");
}


#[tauri::command]
pub fn set_lang(value: String) -> Result<(), Error> {
    system::cfg::set_lang(value)?;
    Ok(())
}

#[tauri::command]
pub fn get_lang() -> Result<Option<String>, Error> {
    system::cfg::get_lang()
}

#[tauri::command]
pub fn set_workspace(value: String) -> Result<(), Error> {
    system::cfg::set_workspace(value)
}

#[tauri::command]
pub fn get_workspace() -> Result<Option<String>, Error> {
    system::cfg::get_workspace()
}

#[tauri::command]
pub fn set_window(window: Window<Wry>) {
    system::set_window_style(&window)
}

#[tauri::command]
pub async fn init_core() -> Result<(), Error> {
    service::core::init().await
}

#[tauri::command]
pub async fn create_drawer(title: String) -> Result<(), Error> {
    service::drawer::create(title).await
}

#[tauri::command]
pub async fn search_drawer(keyword: String) -> Vec<Drawer> {
    service::drawer::search(keyword).await
}

#[tauri::command]
pub async fn change_current_drawer(id: i64) -> Result<(), Error> {
    service::drawer::change_current(id).await
}

#[tauri::command]
pub async fn find_current_drawer() -> Result<Drawer, Error> {
    service::drawer::find_current().await
}

#[tauri::command]
pub async fn remove_folder(id: i64) -> Result<(), Error> {
    service::folder::remove(id).await
}

#[tauri::command]
pub async fn create_subfolder(title: String, intro: String, start_id: i64, link_title: String) -> Result<(), Error> {
    service::folder::create_subset(title, intro, start_id, link_title).await
}

#[tauri::command]
pub async fn link_folder(start_id: i64, end_id: i64, title: String) -> Result<(), Error> {
    service::folder::link(start_id, end_id, title).await
}

#[tauri::command]
pub async fn unlink_folder(relation_id: i64) -> Result<(), Error> {
    service::folder::unlink(relation_id).await
}

#[tauri::command]
pub async fn create_top_level_folder(drawer_id: i64, title: String, intro: String) -> Result<(), Error> {
    service::folder::create_top_level(drawer_id, title, intro).await
}

#[tauri::command]
pub async fn set_as_top_level_folder(relation_id: i64) -> Result<(), Error> {
    service::folder::set_as_top_level(relation_id).await
}

#[tauri::command]
pub async fn modify_folder_title(id: i64, title: String) -> Result<(), Error> {
    service::folder::modify_title(id, title).await
}

#[tauri::command]
pub async fn modify_folder_intro(id: i64, intro: String) -> Result<(), Error> {
    service::folder::modify_intro(id, intro).await
}

#[tauri::command]
pub async fn find_folder(id: i64) -> Result<Folder, Error> {
    service::folder::find_by_id(id).await
}

#[tauri::command]
pub async fn find_folder_belong_drawer(folder_id: i64) -> Result<Drawer, Error> {
    let fd = service::folder::find_folder_drawer(folder_id).await?;
    Ok(service::drawer::find_by_id(fd.drawer_id).await?)
}

#[tauri::command]
pub async fn find_drawer_top_level_folders(drawer_id: i64) -> Result<Vec<ResultItem>, Error> {
    let mut items = vec![];
    for item in service::folder::find_all_top_level(drawer_id).await? {
        let folder = service::folder::find_by_id(item.end_id).await?;
        // let favorite = service::favorites::find(folder.id, "folder".to_string()).await?;
        items.push(ResultItem {
            id: folder.id,
            is_article: false,
            index: None,
            title: folder.title,
            intro: item.title,
            favorite: None,
            sequece_id: None,
            relation_id: Some(item.id),
        })
    }
    Ok(items)
}

#[tauri::command]
pub async fn find_subfolders(folder_id: i64) -> Result<Vec<FolderRelationItem>, Error> {
    let mut items = vec![];
    for item in service::folder::find_all_relations(folder_id, true).await? {
        let folder = service::folder::find_by_id(item.end_id).await?;
        items.push(FolderRelationItem {
            id: folder.id,
            relation_id: item.id,
            title: folder.title,
            label: item.title,
            linked_at: item.created_at,
        })
    }
    Ok(items)
}

#[tauri::command]
pub async fn find_parent_folders(folder_id: i64) -> Result<Vec<Folder>, Error> {
    let mut items = vec![];
    for item in service::folder::find_all_relations(folder_id, false).await? {
        if let Some(start_id) = item.start_id {
            items.push(service::folder::find_by_id(start_id).await?)
        }
    }
    Ok(items)
}

#[tauri::command]
pub async fn find_relation_by_id(id: i64) -> Result<FolderRelation, Error> {
    service::folder::find_relation_by_id(id).await
}

#[tauri::command]
pub async fn find_relation(start_id: i64, end_id: i64) -> Result<Option<FolderRelation>, Error> {
    service::folder::find_relation(start_id, end_id).await
}

#[tauri::command]
pub async fn modify_relation_title(title: String, relation_id: i64) -> Result<(), Error> {
    service::folder::modify_relation_title(title, relation_id).await
}

#[tauri::command]
pub async fn find_relation_map(root_id: Option<i64>, up_deep: Option<i32>, down_deep: Option<i32>) -> Result<FolderMap, Error> {
    service::folder::find_relation_map(root_id, up_deep, down_deep).await
}

#[tauri::command]
pub async fn modify_drawer_title(title: String, id: i64) -> Result<(), Error> {
    service::drawer::modify_title(title, id).await
}

#[tauri::command]
pub async fn modify_drawer_intro(intro: String, id: i64) -> Result<(), Error> {
    service::drawer::modify_intro(intro, id).await
}

#[tauri::command]
pub async fn create_article(title: String, folder_id: i64) -> Result<i64, Error> {
    service::article::create(title, folder_id).await
}

#[tauri::command]
pub async fn remove_article(id: i64) -> Result<(), Error> {
    service::article::remove(id).await
}

#[tauri::command]
pub async fn find_article_by_id(id: i64) -> Result<Article, Error> {
    service::article::find_by_id(id).await
}

#[tauri::command]
pub async fn modify_article_title(id: i64, title: String) -> Result<(), Error> {
    service::article::modify_title(id, title).await
}

#[tauri::command]
pub async fn modify_article_content(id: i64, content: String) -> Result<(), Error> {
    service::article::modify_content(id, content).await
}

#[tauri::command]
pub async fn modify_article_intro(id: i64, intro: String) -> Result<(), Error> {
    service::article::modify_intro(id, intro).await
}

#[tauri::command]
pub async fn toggle_article_editing(id: i64, editing: bool) -> Result<(), Error> {
    service::article::toggle_editing(id, editing).await
}

#[tauri::command]
pub async fn article_link_folder(article_id: i64, folder_id: i64) -> Result<(), Error> {
    service::article::link_folder(article_id, folder_id).await
}

#[tauri::command]
pub async fn article_unlink_folder(article_id: i64, folder_id: i64) -> Result<(), Error> {
    service::article::unlink_folder(article_id, folder_id).await
}

#[tauri::command]
pub async fn find_article_folders(article_id: i64) -> Result<Vec<ArticleLinkedFolderItem>, Error> {
    let mut items = vec![];
    for af in service::article::find_article_folders(article_id).await? {
        let folder = service::folder::find_by_id(af.folder_id).await?;
        items.push(ArticleLinkedFolderItem {
            id: af.id,
            linked_at: af.created_at,
            remark: af.remark,
            title: folder.title,
            folder_id: folder.id,
            article_id,
        })
    }
    Ok(items)
}

#[tauri::command]
pub async fn modify_article_folder_remark(id: i64, remark: String) -> Result<(), Error> {
    service::article::modify_folder_remark(id, remark).await?;
    Ok(())
}

#[tauri::command]
pub async fn find_folder_articles(folder_id: i64) -> Result<Vec<ArticleItem>, Error> {
    let mut items = vec![];
    for af in service::article::find_folder_articles(folder_id).await? {
        let article = service::article::find_by_id(af.article_id).await?;
        items.push(ArticleItem {
            id: af.id,
            title: article.title,
        })
    }
    Ok(items)
}


#[tauri::command]
pub async fn search_folder_articles(folder_id: i64, keyword: String) -> Result<Vec<Article>, Error> {
    service::article::search_folder_articles(folder_id, keyword).await
}

#[tauri::command]
pub async fn add_article_favorite(article_id: i64) -> Result<(), Error> {
    service::article::add_favorite(article_id).await
}

#[tauri::command]
pub async fn find_article_favorite(article_id: i64) -> Result<Option<ArticleFavorite>, Error> {
    service::article::find_favorite(article_id).await
}

#[tauri::command]
pub async fn remove_article_favorite(article_id: i64) -> Result<(), Error> {
    service::article::remove_favorite(article_id).await
}

#[tauri::command]
pub async fn find_all_article_favorites() -> Result<Vec<FavoriteItem>, Error> {
    let mut items = vec![];
    for article_favorite in service::article::find_all_favorites().await? {
        let article = service::article::find_by_id(article_favorite.article_id).await?;
        items.push(FavoriteItem {
            id: article_favorite.id,
            article_id: article_favorite.article_id,
            title: article.title,
            intro: article.intro,
            timestamp: article_favorite.timestamp,
        })
    }
    Ok(items)
}


#[tauri::command]
pub async fn push_article_record(artice_id: i64) -> Result<(), Error> {
    service::record::push_article(artice_id).await
}

#[tauri::command]
pub async fn push_folder_record(folder_id: i64) -> Result<(), Error> {
    service::record::push_folder(folder_id).await
}

#[tauri::command]
pub async fn remove_all_record() -> Result<(), Error> {
    service::record::remove_all().await
}

#[tauri::command]
pub async fn find_all_record() -> Result<Vec<RecordItem>, Error> {
    let mut items = vec![];
    for record in service::record::find_all().await? {
        let title = match record.target_type.as_str() {
            "article" => service::article::find_by_id(record.target_id).await?.title,
            "folder" => service::folder::find_by_id(record.target_id).await?.title,
            _ => String::new()
        };
        items.push(RecordItem {
            id: record.id,
            target_type: record.target_type,
            target_id: record.target_id,
            title,
        })
    }
    
    Ok(items)
}

#[tauri::command]
pub async fn find_folder_subsets(folder_id: i64, subset_type: Option<String>, sort_by_idx: bool) -> Result<Vec<ResultItem>, Error> {
    let mut items = vec![];
    
    for sequence in sequence::search(folder_id, subset_type, sort_by_idx).await? {
        let mut is_article = false;
        let mut title = String::new();
        let mut intro = String::new();
        let mut favorite = None;
        
        if sequence.relation_type == "article".to_string() {
            is_article = true;
            let article = article::find_by_id(sequence.target_id).await?;
            title = article.title;
            intro = article.intro;
            favorite = article::find_favorite(sequence.target_id).await?;
        } else {
            let folder = folder::find_by_id(sequence.target_id).await?;
            title = folder.title;
            if let Some(folder_relation) = folder::find_relation(folder_id, sequence.target_id).await? {
                intro = folder_relation.title;
            }
        }
        // let favorite = favorites::find(sequence.target_id, sequence.relation_type).await?;
        
        items.push(ResultItem {
            id: sequence.target_id,
            is_article,
            index: Some(sequence.idx),
            title,
            intro,
            favorite,
            sequece_id: Some(sequence.id),
            relation_id: Some(sequence.relation_id),
        });
    }
    
    
    Ok(items)
}

#[tauri::command]
pub async fn find_folder_cascader(drawer_id: i64) -> Result<Vec<FolderCascader>, Error> {
    service::folder::find_folder_cascader(drawer_id).await
}

#[tauri::command]
pub async fn modify_sequence_index(id: i64, index: i32) -> Result<(), Error> {
    sequence::modify_index(id, index).await
}

#[tauri::command]
pub async fn folder_counter(folder_id: i64) -> Result<CountItem, Error> {
    folder::count(folder_id).await
}


