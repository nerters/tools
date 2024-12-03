use std::sync::atomic::Ordering;
use idgen::IDGen;
use tauri::{Emitter, Manager};

use crate::{dao::cron::{self, alert_win, delete_by_id, save, update_tokio, update_use_tokio, CronInfo, CRON_MAP}, utils::date_util::get_now_time_m};

use super::other_serve::keyboard_light;

#[tauri::command]
pub async fn savn_cron(
    name: String,
    content: String,
    cron_type: String,
    interval: i64,
    appointed_time: i64,
    category: String,
    pid: String,
) -> i64 {
    let idgen = IDGen::new(1);
    let id = idgen.new_id();
    let mut info = CronInfo::default();
    info.id = id.to_string();
    info.name = name;
    info.content = content;
    info.cron_type = cron_type;
    info.interval = interval;
    info.appointed_time = appointed_time;
    info.category = category.clone();
    info.pid = pid;
    save(info.clone()).await;
    if category == "cron" {
        let mut map = CRON_MAP.lock().await;
        map.insert(id.to_string(), info);
    }
    id as i64
}

#[tauri::command]
pub fn update_cron(
    id: String,
    name: String,
    content: String,
    interval: i64,
    appointed_time: i64,
    pid: String,
    sort: i16,
    category: String,
    is_use: i64,
    cron_type: String,
    activity: i64,
) {
    let mut info = CronInfo::default();
    info.id = id;
    info.name = name;
    info.content = content;
    info.interval = interval;
    info.appointed_time = appointed_time;
    info.pid = pid;
    info.sort = sort;
    info.category = category;
    info.is_use = is_use;
    info.cron_type = cron_type;
    info.activity = activity;
    println!("更新");
    update_tokio(info);
}

#[tauri::command]
pub async fn get_list_cron() -> Vec<CronInfo> {
    cron::get_list_cache().await
}

#[tauri::command]
pub async fn get_tree_cron() -> Vec<CronInfo> {
    cron::get_tree().await
}

#[tauri::command]
pub async fn del_cron(id: String) {
    delete_by_id(id.clone()).await;
    let mut map = CRON_MAP.lock().await;
    println!("删除id:{}", id.clone());
    map.remove(&id);
    match serde_json::to_string(&map.clone()) {
        Ok(x) => println!("删除后的数据:{}", x),
        Err(_) => todo!(),
    };
}

#[tauri::command]
pub async fn open_docs(handle: tauri::AppHandle) {
    alert_win(handle.clone()).await;
    //floating_window(handle.clone()).await;
}

#[tauri::command]
pub async fn use_cron(handle: tauri::AppHandle, id: String) {
    if !id.is_empty() {
        let mut map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&id) {
            let mut temp = temp.clone();
            temp.update_time = get_now_time_m();
            temp.is_use = 0;
            map.insert(id.clone(), temp.clone());
            println!("cron更新缓存 : {}", temp.name);
            let mut win_key = "countDown-".to_string() + &id.clone();

            if let Some(window) = handle.get_webview_window("main") {
                //let list: Vec<CronInfo> = map.values().cloned().collect();
                //window.emit("get_list_cron", list).unwrap();
                window.emit("get_cron_info", temp.clone()).unwrap();
            }

            if let Some(window) = handle.get_webview_window(&win_key) {
                //let list: Vec<CronInfo> = map.values().cloned().collect();
                //window.emit("get_list_cron", list).unwrap();
                window.emit("get_cron_info", temp).unwrap();
            }
        }
        //更新数据库
        let mut info = CronInfo::default();
        info.id = id;
        info.is_use = 0;
        info.update_time = get_now_time_m();
        update_use_tokio(info).await;
        cron::CRON_UPDATE.store(true, Ordering::SeqCst);
    }
}


#[tauri::command]
pub async fn stop_cron(handle: tauri::AppHandle, id: String) {
    if !id.is_empty() {
        let mut map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&id) {
            let mut temp = temp.clone();
            temp.update_time = get_now_time_m();
            temp.activity = 0;
            temp.is_use = 0;
            map.insert(id.clone(), temp.clone());
            println!("cron更新缓存 : {}", temp.name);
            let mut win_key = "countDown-".to_string() + &id.clone();

            if let Some(window) = handle.get_webview_window("main") {
                //let list: Vec<CronInfo> = map.values().cloned().collect();
                //window.emit("get_list_cron", list).unwrap();
                window.emit("get_cron_info", temp.clone()).unwrap();
            }

            if let Some(window) = handle.get_webview_window(&win_key) {
                //let list: Vec<CronInfo> = map.values().cloned().collect();
                //window.emit("get_list_cron", list).unwrap();
                window.emit("get_cron_info", temp).unwrap();
            }
        }
        //更新数据库
        let mut info = CronInfo::default();
        info.id = id;
        info.activity = 0;
        info.update_time = get_now_time_m();
        cron::stop_cron(info).await;
        cron::CRON_UPDATE.store(true, Ordering::SeqCst);
    }
}





#[tauri::command]
pub async fn get_cron_info(id: String) -> CronInfo {
    println!("get_cron_info参数:{}", id);
    if !id.is_empty() {
        let map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&id) {
            let temp = temp.clone();
            println!("******{}", temp.update_time);
            return temp;
        } else {
            println!("没有查询到{}", id);
        }
    }
    return CronInfo::default();
}



#[tauri::command]
pub async fn floating_window(handle: tauri::AppHandle, id: String) {
    let mut win_key = "countDown-".to_string();
    let mut info_id = String::from("");
    let height = 100.0;
    let mut width = 288.0;
    let map = CRON_MAP.lock().await;
    if let Some(temp) = map.get(&id) {
        win_key += &temp.id.clone();
        info_id += &temp.id.clone();
        if temp.category != "cron" {
            return;
        }
        if temp.interval > 86400 || temp.appointed_time - get_now_time_m() > 86400 {
            width = 388.0;
        }
    }

    if let Some(_win) = handle.get_webview_window(&win_key) {
        println!("窗口已启动!");
    } else {
        let win_list = handle.webview_windows();
        let mut win_num = 0;
        for (key, _value) in win_list {
            if key.starts_with("countDown") {
                win_num += 1;
            }
        }
        let docs_window = tauri::WebviewWindowBuilder::new(
            &handle,
            win_key, /* the unique window label */
            tauri::WebviewUrl::App("/hint/CountDown".parse().unwrap()),
        )
        .title(info_id)
        .inner_size(width, height)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .skip_taskbar(true)
        .focused(false)
        .always_on_top(true)
        .position(800.0, 100.0 + (win_num as f64) * 100.0)
        .build();

        match docs_window {
            Ok(win) => {

            }
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }
}