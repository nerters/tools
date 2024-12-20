use idgen::IDGen;

use crate::dao::hot_key::{self, HotKey};

#[tauri::command]
pub fn get_hot_key_list() -> Vec<HotKey> {
   
    hot_key::get_list()
}

#[tauri::command]
pub fn add_hot_key(_handle: tauri::AppHandle, key: String, path: String, desc: String, overopen: i64, url: String, shell: String) {
    let mut info = HotKey::default();
    let idgen = IDGen::new(1);
    let id = idgen.new_id();
    info.id = id.to_string();
    info.key = key;
    info.path = path;
    info.desc = desc;
    info.url = url;
    info.shell = shell;
    info.overopen = overopen;
    let _ = hot_key::save(info.clone());
}

#[tauri::command]
pub fn update_hot_key(handle: tauri::AppHandle, id: String, key: String, path: String, desc: String, overopen: i64, url: String, shell: String) {
    let mut info = HotKey::default();
    info.id = id;
    info.key = key;
    info.path = path;
    info.desc = desc;
    info.url = url;
    info.shell = shell;
    info.overopen = overopen;
    let _ = hot_key::update(info);
}

#[tauri::command]
pub fn delete_hot_key(handle: tauri::AppHandle, id: String) {
    let _ = hot_key::delete_by_id(id);
    #[cfg(all(desktop))]
    {
        if handle.remove_plugin("global-shortcut")  {
            println!("成功global-shortcut");
        } else {
            println!("失败global-shortcut");
        }  
    }

}