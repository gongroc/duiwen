use std::collections::HashSet;
use std::hash::Hash;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};
use crate::model::entity::{ArticleFavorite};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FolderMap {
    pub nodes: HashSet<Node>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub source: i64,
    pub target: i64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCascader {
    // 文件夹编号
    pub value: i64,
    // 文件夹名
    pub label: String,
    // 子集文件夹
    pub children: Vec<FolderCascader>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleItem {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderItem {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderRelationItem {
    pub id: i64,
    pub relation_id: i64,
    pub title: String,
    pub label: String,
    pub linked_at: FastDateTime,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleLinkedFolderItem {
    pub id: i64,
    pub linked_at: FastDateTime,
    pub remark: String,
    pub title: String,
    pub folder_id: i64,
    pub article_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordItem {
    pub id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItem {
    pub id: i64,
    pub article_id: i64,
    pub title: String,
    pub intro: String,
    pub timestamp: FastDateTime,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultItem {
    // 文章或文件夹编号
    pub id: i64,
    // 是否为文章
    pub is_article: bool,
    // 排序编号
    pub index: Option<i32>,
    // 文章标题或文件夹名
    pub title: String,
    // 文章简介或文件夹关联名
    pub intro: String,
    // 收藏信息
    pub favorite: Option<ArticleFavorite>,
    // 排序表编号
    pub sequece_id: Option<i64>,
    // 文件夹关系列表
    pub relation_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountItem {
    pub article: usize,
    pub folder: usize,
}


