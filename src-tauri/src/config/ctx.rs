use crate::config::error::Error;
use once_cell::sync::OnceCell;
use rbatis::Rbatis;
use rbdc_sqlite::driver::SqliteDriver;
use std::sync::Arc;
use rust_i18n::t;

static INSTANCE: OnceCell<CtxManager> = OnceCell::new();

#[derive(Debug)]
pub struct CtxManager {
    pub sqlite: Arc<Rbatis>,
}

impl CtxManager {
    /// 实例化CtxManager单利对象
    pub async fn init(workspace: String) -> Result<(), Error> {
        let sqlite = Arc::new(Self::sqlite(workspace).await?);
        let _ = INSTANCE.set(CtxManager { sqlite });
        Ok(())
    }

    /// 初始化sqlite
    async fn sqlite(workspace: String) -> Result<Rbatis, Error> {
        let rb = Rbatis::new();
        let db_file_path = std::path::Path::new(workspace.as_str()).join("data.db");
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(!db_file_path.exists())
            .open(&db_file_path)?;
        if let Some(file_path) = db_file_path.to_str() {
            if let Err(err) = rb.init(SqliteDriver {}, format!("sqlite:{file_path}").as_str()) {
                log::error!("ctx_sqlite:{}",err);
                return Err(Error::System(t!("failedToConnectToDatabase")));
            } else {
                let sql = include_str!("../../database.sql");
                let _ = rb.exec(sql, vec![]).await.unwrap();
            }
        }
        return Ok(rb);
    }

    /// 返回当前对象单利实例
    pub fn get() -> &'static Self {
        INSTANCE.get().expect("上下文未初始化")
    }
}
