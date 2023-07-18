use crate::config::error::Error;
use crate::system::cfg::{get_lang, set_lang};
use crate::system::dir::get_base_dir_path;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::ZipPacker;
use sys_locale::get_locale;

/// 初始化国际化语言
/// 如果未配置则取系统语言环境
/// 只支持中文与英文两种
pub fn i18n() -> Result<(), Error> {
    let mut lang = String::new();
    if let Some(str) = get_lang()? {
        lang = str
    } else {
        let mut locale = get_locale().unwrap_or_else(|| String::from("en-US"));
        if locale != "en-US" {
            locale = String::from("en-US");
        }
        set_lang(locale.clone())?;
        lang = locale;
    }
    rust_i18n::set_locale(lang.as_str());
    Ok(())
}

/// 初始化日志配置
/// 存放在程序根目录下logs文件夹内
/// 存储的单个文件以5M大小进行切割，压缩为zip格式
pub fn log() -> Result<(), Error> {
    let log_dir = get_base_dir_path()?.join("logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)?;
    }
    if let Some(log_dir) = log_dir.to_str() {
        fast_log::init(fast_log::Config::new().console().file_split(
            format!("{log_dir}/").as_str(),
            LogSize::MB(5),
            RollingType::All,
            ZipPacker {},
        ))?;
    }
    Ok(())
}
