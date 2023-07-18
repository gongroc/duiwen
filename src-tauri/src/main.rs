#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod cmd;
mod config;
mod event;
mod service;
mod system;
mod utils;
mod model;

use crate::system::init;
use cmd::ipc;
use cmd::push;
use tauri::Manager;

rust_i18n::i18n!("locales");

fn main() {
    init();
    tauri::Builder::default()
        .setup(|app| {
            for (_, window) in app.windows() {
                system::set_window_style(&window)
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::set_window,
            ipc::set_lang,
            ipc::get_lang,
            ipc::set_workspace,
            ipc::get_workspace,
            ipc::init_core,
            ipc::test,
            ipc::create_drawer,
            ipc::search_drawer,
            ipc::change_current_drawer,
            ipc::find_current_drawer,
            ipc::remove_folder,
            ipc::create_subfolder,
            ipc::link_folder,
            ipc::unlink_folder,
            ipc::create_top_level_folder,
            ipc::set_as_top_level_folder,
            ipc::modify_folder_title,
            ipc::modify_folder_intro,
            ipc::find_folder,
            ipc::find_folder_belong_drawer,
            ipc::find_drawer_top_level_folders,
            ipc::find_subfolders,
            ipc::find_parent_folders,
            ipc::find_relation_by_id,
            ipc::find_relation,
            ipc::modify_relation_title,
            ipc::find_relation_map,
            ipc::modify_drawer_title,
            ipc::modify_drawer_intro,
            ipc::create_article,
            ipc::remove_article,
            ipc::find_article_by_id,
            ipc::modify_article_title,
            ipc::modify_article_content,
            ipc::modify_article_intro,
            ipc::toggle_article_editing,
            ipc::article_link_folder,
            ipc::article_unlink_folder,
            ipc::find_article_folders,
            ipc::modify_article_folder_remark,
            ipc::find_folder_articles,
            ipc::search_folder_articles,
            ipc::add_article_favorite,
            ipc::find_article_favorite,
            ipc::remove_article_favorite,
            ipc::find_all_article_favorites,
            ipc::push_article_record,
            ipc::push_folder_record,
            ipc::remove_all_record,
            ipc::find_all_record,
            ipc::find_folder_subsets,
            ipc::find_folder_cascader,
            ipc::modify_sequence_index,
            ipc::folder_counter,
            push::message_hub
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
