/// 当语言被设置后，切换系统程序国际化设置的语言环境
pub fn change_local_when_lang_setted(lang: String) {
    rust_i18n::set_locale(lang.as_str());
}
