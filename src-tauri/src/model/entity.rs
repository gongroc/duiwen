use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Serialize, Deserialize, Deserializer};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub intro: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub editing: bool,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,
}

crud!(Article{},"Article");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleFavorite {
    pub id: i64,
    pub article_id: i64,
    pub timestamp: FastDateTime,
}

crud!(ArticleFavorite{},"ArticleFavorite");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleFolder {
    pub id: i64,
    pub folder_id: i64,
    pub article_id: i64,
    pub remark: String,
    pub created_at: FastDateTime,
}

crud!(ArticleFolder{},"ArticleFolder");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: i64,
    pub title: String,
    pub intro: Option<String>,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,
}

crud!( Folder{},"Folder");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderRelation {
    pub id: i64,
    pub start_id: Option<i64>,
    pub end_id: i64,
    pub title: String,
    pub created_at: FastDateTime,
}

crud!( FolderRelation{},"FolderRelation");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Drawer {
    pub id: i64,
    pub title: String,
    pub intro: Option<String>,
    pub idx: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub current: bool,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,
}

crud!(Drawer{},"Drawer");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderDrawer {
    pub id: i64,
    pub folder_id: i64,
    pub drawer_id: i64,
    pub created_at: FastDateTime,
}

crud!(FolderDrawer{},"FolderDrawer");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: i64,
    pub target_id: i64,
    pub target_type: String,
    pub created_at: FastDateTime,
}

crud!(Record{},"Record");


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sequence {
    pub id: i64,
    // 文件夹编号
    pub folder_id: i64,
    // 文件夹关系或文章所在文件夹的关系编号
    pub relation_id: i64,
    // 关系类型，文件夹或文章
    pub relation_type: String,
    // 文章或文件夹编号
    pub target_id: i64,
    // 文章或文件夹名称
    pub title: String,
    // 排序
    pub idx: i32,
    // 最后修改时间
    pub timestamp: FastDateTime,
}

crud!(Sequence{},"Sequence");

/// 反序列化
/// NOTE： 按照SQLITE的规范，数据库存INT，1为true，0为false，
/// 但是rbatis没有根据实体与数据库类型转换，数据库值为true时候，他依然当字符串处理
/// 所以这里反序列化的时候，按照字符串处理
pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    let i = String::deserialize(deserializer)?.to_lowercase();
    Ok(i == String::from("true"))
}