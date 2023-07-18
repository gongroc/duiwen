#[warn(unused_imports)]
use rbatis::executor::Executor;
use rbatis::{py_sql, Rbatis};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use crate::model::entity::{Article, ArticleFavorite, ArticleFolder, Drawer, Folder, FolderDrawer, FolderRelation, Record, Sequence};

#[py_sql("insert into Drawer(title,idx) values(#{title},#{idx})")]
pub async fn insert_drawer(rb: &mut dyn Executor, title: &str, idx: &i32) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`select * from Drawer where 1 = 1 `
            if keyword != '':
                and title like '%' || #{keyword} || '%'
            ` order by 'idx' desc`")]
pub async fn search_drawer(rb: &Rbatis, keyword: &str) -> Result<Vec<Drawer>, Error> { todo!() }

#[py_sql("select MAX(idx) from Drawer")]
pub async fn select_drawer_max_index(rb: &Rbatis) -> Result<Option<i32>, Error> { todo!() }

#[py_sql("`select * from Drawer where 1 = 1 `
            if id != '':
                `and id = #{id} `
            if current != '':
                and current = #{current} ")]
pub async fn select_one_drawer(rb: &Rbatis, id: &str, current: &str) -> Result<Option<Drawer>, Error> { todo!() }

#[py_sql("`update Drawer set  `
            if title != '':
                title = #{title},
            if intro != '':
                intro = #{intro},
            if idx != '':
                idx = #{idx},
            if current != '':
                current = #{current},
            ` updated_at = current_timestamp where id = #{id}`")]
pub async fn update_drawer(rb: &mut dyn Executor, title: &str, intro: &str, idx: &str, current: &str, id: &i64) -> Result<ExecResult, Error> { todo!() }

#[py_sql("select count(*) from Drawer")]
pub async fn count_all_drawer(rb: &Rbatis) -> Result<i32, Error> { todo!() }

#[py_sql("select * from Folder where id = #{id}")]
pub async fn select_one_folder(rb: &Rbatis, id: &i64) -> Result<Option<Folder>, Error> { todo!() }

#[py_sql("insert into Folder(title,intro) values(#{title},#{intro})")]
pub async fn insert_folder(rb: &mut dyn Executor, title: &str, intro: &str) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`update Folder set `
            if title != '':
                title = #{title},
            if intro != '':
                intro = #{intro},
            ` updated_at = #{timestamp} where id = #{id}`")]
pub async fn update_folder(rb: &mut dyn Executor, title: &str, intro: &str, timestamp: &str, id: &i64) -> Result<ExecResult, Error> { todo!() }

#[py_sql("insert into FolderRelation(
            if start_id != '':
                start_id,
            end_id,title) values(
            if start_id != '':
                #{start_id},
            #{end_id},#{title})")]
pub async fn insert_folder_relation(rb: &mut dyn Executor, start_id: &str, end_id: &i64, title: &str) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`select * from FolderRelation where 1 = 1 `
            if start_id != '':
                `and start_id = #{start_id} `
            if end_id != '':
                `and end_id = #{end_id} `
            if id != '':
                and id = #{id}")]
pub async fn select_one_folder_relation(rb: &Rbatis, start_id: &str, end_id: &str, id: &str) -> Result<Option<FolderRelation>, Error> { todo!() }

#[py_sql("`select * from FolderRelation where 1 = 1 `
            if start_id != '':
                `and start_id = #{start_id} `
            if end_id != '':
                and end_id = #{end_id}")]
pub async fn select_all_folder_relations(rb: &Rbatis, start_id: &str, end_id: &str) -> Result<Vec<FolderRelation>, Error> { todo!() }

#[py_sql("`update FolderRelation set `
            trim ',':
                if start_id != '':
                    choose :
                        when start_id == 'NULL':
                            start_id = NULL
                        otherwise :
                            start_id = #{start_id},
                if end_id != '':
                    end_id = #{end_id},
                if title != '':
                    title = #{title},
            ` where id = #{id}`")]
pub async fn update_folder_relation(rb: &mut dyn Executor, id: &i64, start_id: &str, end_id: &str, title: &str) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`SELECT fr.* `
            `FROM `
                `FolderRelation AS fr `
                `JOIN Folder AS f ON f.id = fr.end_id `
                `JOIN FolderDrawer AS fd ON fd.folder_id = f.id `
            `WHERE `
                `fr.start_id IS NULL `
            ` AND fd.drawer_id = #{drawer_id} `")]
pub async fn select_drawer_top_level_folders(rb: &Rbatis, drawer_id: &i64) -> Result<Vec<FolderRelation>, Error> { todo!() }

#[py_sql("insert into FolderDrawer(folder_id,drawer_id) values(#{folder_id},#{drawer_id})")]
pub async fn insert_folder_drawer(rb: &mut dyn Executor, folder_id: &i64, drawer_id: &i64) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`select * from FolderDrawer where 1 = 1 `
            if id != '':
                `and id = #{id} `
            if folder_id != '':
                and folder_id = #{folder_id}")]
pub async fn select_one_folder_drawer(rb: &Rbatis, id: &str, folder_id: &str) -> Result<Option<FolderDrawer>, Error> { todo!() }

#[py_sql("select * from FolderDrawer where 1 = 1
            if drawer_id != '':
                and drawer_id = #{drawer_id}")]
pub async fn select_all_drawer_folders(rb: &Rbatis, drawer_id: &str) -> Result<Vec<FolderDrawer>, Error> { todo!() }

#[py_sql("select * from Article where id = #{id}")]
pub async fn select_one_article(rb: &Rbatis, id: &i64) -> Result<Option<Article>, Error> { todo!() }

#[py_sql("insert into Article(title) values(#{title})")]
pub async fn insert_article(rb: &mut dyn Executor, title: &str) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`update Article set `
            if title != '':
                title = #{title},
            if content != '':
                content = #{content},
            if editing != '':
                editing = #{editing},
            if intro != '':
                intro = #{intro},
            ` updated_at = current_timestamp  where id = #{id} `")]
pub async fn update_article(rb: &mut dyn Executor, title: &str, content: &str, intro: &str, editing: &str, id: &i64) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`delete from ArticleFolder where 1 = 1 `
            if folder_id != '':
                `and folder_id = #{folder_id} `
            if article_id != '':
                `and article_id = #{article_id} `")]
pub async fn delete_article_folder(rb: &mut dyn Executor, folder_id: &str, article_id: &str) -> Result<ExecResult, Error> { todo!() }


#[py_sql("insert into ArticleFolder(folder_id,article_id) values(#{folder_id},#{article_id})")]
pub async fn insert_article_folder(rb: &mut dyn Executor, folder_id: &i64, article_id: &i64) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`select * from ArticleFolder where 1 = 1 `
            if folder_id != '':
                `and folder_id = #{folder_id} `
            if article_id != '':
                `and article_id = #{article_id} `
            if id !='':
                and id = #{id}")]
pub async fn select_one_article_folder(rb: &Rbatis, id: &str, folder_id: &str, article_id: &str) -> Result<Option<ArticleFolder>, Error> { todo!() }

#[py_sql("`select * from ArticleFolder where 1 = 1 `
            if folder_id != '':
                `and folder_id = #{folder_id} `
            if article_id != '':
                `and article_id = #{article_id} `
            ` order by id desc`")]
pub async fn select_all_article_folder(rb: &Rbatis, folder_id: &str, article_id: &str) -> Result<Vec<ArticleFolder>, Error> { todo!() }


#[py_sql("update ArticleFolder set remark = #{remark} where id = #{id}")]
pub async fn update_article_folder(rb: &mut dyn Executor, id: &i64, remark: &str) -> Result<ExecResult, Error> { todo!() }

#[py_sql("select a.* from Article as a join ArticleFolder as af on af.article_id = a.id where
            af.folder_id = #{folder_id}
            and a.title like '%' || #{keyword} || '%'
            order by af.idx desc")]
pub async fn search_article_folder(rb: &Rbatis, folder_id: &i64, keyword: &str) -> Result<Vec<Article>, Error> { todo!() }

#[py_sql("insert into ArticleFavorite(article_id) values(#{article_id})")]
pub async fn insert_article_favorite(rb: &mut dyn Executor, article_id: &i64) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`select * from ArticleFavorite where 1 = 1 `
            if id != '':
                `and id = #{id} `
            if article_id != '':
                `and article_id = #{article_id} `
")]
pub async fn select_one_article_favorite(rb: &Rbatis, id: &str, article_id: &str) -> Result<Option<ArticleFavorite>, Error> { todo!() }

#[py_sql("select * from ArticleFavorite order by timestamp desc")]
pub async fn select_all_article_favorite(rb: &Rbatis) -> Result<Vec<ArticleFavorite>, Error> { todo!() }

#[py_sql("update ArticleFavorite set timestamp = #{timestamp} where article_id = #{article_id}")]
pub async fn update_article_favorite(rb: &mut dyn Executor, article_id: &i64, timestamp: &FastDateTime) -> Result<ExecResult, Error> { todo!() }

#[py_sql("insert into Record(target_id,target_type) values(#{target_id},#{target_type})")]
pub async fn insert_record(rb: &mut dyn Executor, target_id: &i64, target_type: &str) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`select * from Record where 1 = 1 `
            if target_id != '':
                `and target_id = #{target_id} `
            if target_type != '':
                `and target_type = #{target_type} `
            ` order by id desc`")]
pub async fn select_all_record(rb: &Rbatis, target_id: &str, target_type: &str) -> Result<Vec<Record>, Error> { todo!() }

#[py_sql("select * from Record order by id desc limit 0,1 ")]
pub async fn select_last_record(rb: &Rbatis) -> Result<Option<Record>, Error> { todo!() }

#[py_sql("delete from Record where id in(select id from Record order by id desc limit #{limit},100000000)")]
pub async fn delete_record(rb: &mut dyn Executor, limit: &i32) -> Result<ExecResult, Error> { todo!() }


#[py_sql("`delete from Record where 1 = 1 `
            if target_id != '':
                `and target_id = #{target_id} `
            if target_type != '':
                and target_type = #{target_type}")]
pub async fn delete_all_record(rb: &mut dyn Executor, target_id: &str, target_type: &str) -> Result<ExecResult, Error> { todo!() }


// #[py_sql("insert into Favorites(target_id,target_type,idx) values(#{target_id},#{target_type},#{idx})")]
// pub async fn insert_favorites(rb: &mut dyn Executor, target_id: &str, target_type: &str, idx: &i32) -> Result<ExecResult, Error> { todo!() }

// #[py_sql("`select * from Favorites where 1 = 1 `
//             if target_id != '':
//                 `and target_id = #{target_id} `
//             if target_type != '':
//                 and target_type = #{target_type}
//             ` order by idx desc`")]
// pub async fn select_all_favorites(rb: &Rbatis, target_id: &str, target_type: &str) -> Result<Vec<Favorites>, Error> { todo!() }

// #[py_sql("`select * from Favorites where 1 = 1  `
//             if target_id != '':
//                 `and target_id = #{target_id} `
//             if target_type != '':
//                 and target_type = #{target_type}")]
// pub async fn select_one_favorites(rb: &Rbatis, target_id: &str, target_type: &str) -> Result<Option<Favorites>, Error> { todo!() }

// #[py_sql("`delete from Favorites where 1 = 1 `
//             if target_id != '':
//                 `and target_id = #{target_id} `
//             if target_type != '':
//                 and target_type = #{target_type}")]
// pub async fn delete_all_favorites(rb: &mut dyn Executor, target_id: &str, target_type: &str) -> Result<ExecResult, Error> { todo!() }

// #[py_sql("update Favorites set idx = #{idx} where id = #{id}")]
// pub async fn update_favorites(rb: &mut dyn Executor, id: &i64, idx: &i32) -> Result<ExecResult, Error> { todo!() }

// #[py_sql("`select * from Favorites where 1 = 1 `
//             if target_id != '':
//                 `and target_id = #{target_id} `
//             if target_type != '':
//                 `and target_type = #{target_type} `
//             ` order by id desc limit 0,1`")]
// pub async fn select_last_favorites(rb: &Rbatis, target_id: &str, target_type: &str) -> Result<Option<Favorites>, Error> { todo!() }

#[py_sql("insert into Sequence(folder_id,relation_id,relation_type,target_id,title,idx)
            values(#{folder_id},#{relation_id},#{relation_type},#{target_id},#{title},#{idx})")]
pub async fn insert_sequence(rb: &mut dyn Executor, folder_id: &i64, relation_id: &i64, relation_type: &str, target_id: &i64,
                             title: &str, idx: &i32) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`update Sequence set `
            trim ',':
                if title != '':
                    title = #{title},
                if idx != '':
                    idx = #{idx},
                if timestamp != '':
                    timestamp = DateTime('${timestamp}'),
            ` where 1 = 1 `
                if id != '':
                    `and id = #{id} `
                if target_id != '':
                    `and target_id = #{target_id} `
                if relation_type != '':
                    `and relation_type = #{relation_type} `
                if relation_id != '':
                    `and relation_id = #{relation_id} `")]
pub async fn update_sequece(rb: &mut dyn Executor, id: &str, title: &str, idx: &str, timestamp: &str, target_id: &str,
                            relation_type: &str, relation_id: &str) -> Result<ExecResult, Error> { todo!() }

#[py_sql("`delete from Sequence where 1 = 1 `
            if folder_id != '':
                `and folder_id = #{folder_id} `
            if relation_id != '':
                `and relation_id = #{relation_id} `
            if relation_type != '':
                `and relation_type = #{relation_type} `
            if target_id != '':
                `and target_id = #{target_id} `
            ")]
pub async fn delete_sequece(rb: &mut dyn Executor, folder_id: &str, relation_id: &str, relation_type: &str, target_id: &str) -> Result<ExecResult, Error> { todo!() }


#[py_sql("`select * from Sequence where  folder_id = #{folder_id} `
                if keyword != '':
                    `and title like '%' || #{keyword} || '%' `
                if relation_type != '':
                    `and relation_type = #{relation_type} `
            `order by `
                choose:
                    when sort_by_idx:
                        idx
                    otherwise:
                        timestamp
             ` desc`")]
pub async fn search_all_sequece(rb: &Rbatis, keyword: &str, folder_id: &i64, relation_type: &str, sort_by_idx: &bool) -> Result<Vec<Sequence>, Error> { todo!() }

#[py_sql("select * from Sequence where id = #{id}")]
pub async fn select_one_sequece(rb: &Rbatis, id: &i64) -> Result<Option<Sequence>, Error> { todo!() }

#[py_sql("select * from Sequence where folder_id = #{folder_id} order by id desc limit 0,1")]
pub async fn select_last_sequece(rb: &Rbatis, folder_id: &i64) -> Result<Option<Sequence>, Error> { todo!() }

#[py_sql("`select count(*) from ArticleFolder where 1 = 1 `
                if folder_id != '':
                    `and folder_id = #{folder_id}`
                if article_id != '':
                    `and article_id = #{article_id}`")]
pub async fn count_article_folder(rb: &Rbatis, folder_id: &str, article_id: &str) -> Result<i32, Error> { todo!() }

