use idgen::IDGen;

use crate::dao::grid_info::{self, merge_data, GridInfo};

#[tauri::command]
pub async fn grid_merge_data(data_list: Vec<GridInfo>) -> Vec<GridInfo> {
    println!("grid_merge_data");
    merge_data(data_list).await
}

#[tauri::command]
pub async fn add_grid(
    name: String,
    describe: String,
    uri: String,
    code: String,
    classify: String,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
) -> i64 {
    let idgen = IDGen::new(1);
    let id = idgen.new_id();
    let mut info = GridInfo::default();
    info.id = id.to_string();
    info.name = name;
    info.describe = describe;
    info.uri = uri;
    info.code = code;
    info.classify = classify;
    info.is_sys = 0;
    info.x = x;
    info.y = y;
    info.w = w;
    info.h = h;
    grid_info::save(info).await;
    id as i64
}

#[tauri::command]
pub async fn update_grid(
    id: String,
    name: String,
    describe: String,
    uri: String,
    code: String,
    classify: String,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
) -> bool {
    let mut info = GridInfo::default();
    info.id = id.to_string();
    info.name = name;
    info.describe = describe;
    info.uri = uri;
    info.code = code;
    info.classify = classify;
    info.is_sys = 0;
    info.x = x;
    info.y = y;
    info.w = w;
    info.h = h;
    println!("更新");
    grid_info::update(info).await
}

#[tauri::command]
pub async fn delete_grid_by_id(id: String) {
    grid_info::delete_by_id(id).await;
}

#[tauri::command]
pub async fn get_grid_by_id(id: String) -> GridInfo {
    grid_info::get_info_by_id(id).await
}