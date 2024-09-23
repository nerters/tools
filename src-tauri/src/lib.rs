use std::sync::atomic::Ordering;

use dao::{
    cron::{
        self, alert_win, delete_by_id, save, update_tokio, update_use_tokio, CronInfo, CRON_MAP
    },
    grid_info::{self, merge_data, GridInfo}, hot_key::{self, HotKey},
};
use idgen::IDGen;
use img::img_utils::compress;
use tauri::{AppHandle, Emitter, Manager};
use utils::{date_util::get_now_time_m, mysql_utils::init_mysql_pool};
mod dao;
mod img;
mod json;
mod utils;

#[cfg(desktop)]
mod tray;
#[cfg(desktop)]
mod hotKey;

#[tauri::command]
fn compress_img(file_path: String, nwidth: u32, nheight: u32, img_type: String) -> String {
    compress(&file_path, nwidth, nheight, img_type)
}

#[tauri::command]
async fn savn_cron(
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
fn update_cron(
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
async fn get_list_cron() -> Vec<CronInfo> {
    cron::get_list_cache().await
}

#[tauri::command]
async fn get_tree_cron() -> Vec<CronInfo> {
    cron::get_tree().await
}

#[tauri::command]
async fn del_cron(id: String) {
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
async fn open_docs(handle: tauri::AppHandle) {
    alert_win(handle.clone()).await;

    //floating_window(handle.clone()).await;
}

#[tauri::command]
async fn use_cron(handle: tauri::AppHandle, id: String) {
    if !id.is_empty() {
        let mut map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&id) {
            let mut temp = temp.clone();
            temp.update_time = get_now_time_m();
            temp.is_use = 0;
            map.insert(id.clone(), temp.clone());

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
async fn stop_cron(handle: tauri::AppHandle, id: String) {
    if !id.is_empty() {
        let mut map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&id) {
            let mut temp = temp.clone();
            temp.update_time = get_now_time_m();
            temp.activity = 0;
            temp.is_use = 0;
            map.insert(id.clone(), temp.clone());

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
async fn get_cron_info(id: String) -> CronInfo {
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
fn read_file(path: String) -> String {
    println!("{}", path);
    match file::get_text(path) {
        Ok(text) => {
            return text;
        }
        Err(e) => {
            println!("获取数据失败！{}", e);
            return format!("#@#error#@#_{}", e);
        }
    }
}

#[tauri::command]
async fn grid_merge_data(data_list: Vec<GridInfo>) -> Vec<GridInfo> {
    println!("grid_merge_data");
    merge_data(data_list).await
}

#[tauri::command]
async fn add_grid(
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
async fn update_grid(
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
async fn delete_grid_by_id(id: String) {
    grid_info::delete_by_id(id).await;
}

#[tauri::command]
async fn get_grid_by_id(id: String) -> GridInfo {
    grid_info::get_info_by_id(id).await
}

#[tauri::command]
async fn floating_window(handle: tauri::AppHandle, id: String) {
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
        .position(800.0, 100.0 + (win_num as f64) * 100.0)
        .build();

        match docs_window {
            Ok(win) => {
                let _ = win.set_always_on_top(true);
            }
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }
}

#[tauri::command]
fn get_hot_key_list() -> Vec<HotKey> {
   
    hot_key::get_list()
}

#[tauri::command]
fn add_hot_key(handle: tauri::AppHandle, key: String, path: String, desc: String, overopen: i64, url: String) {
    let mut info = HotKey::default();
    let idgen = IDGen::new(1);
    let id = idgen.new_id();
    info.id = id.to_string();
    info.key = key;
    info.path = path;
    info.desc = desc;
    info.url = url;
    info.overopen = overopen;
    let _ = hot_key::save(info.clone());
    #[cfg(all(desktop))]
    {
        let _ = hotKey::add_host_key(&handle, info);

        let _ = hotKey::create_host_key(&handle);
    }
}

#[tauri::command]
fn update_hot_key(handle: tauri::AppHandle, id: String, key: String, path: String, desc: String, overopen: i64, url: String) {
    let mut info = HotKey::default();
    info.id = id;
    info.key = key;
    info.path = path;
    info.desc = desc;
    info.url = url;
    info.overopen = overopen;
    let _ = hot_key::update(info);
    #[cfg(all(desktop))]
    {
        let _ = hotKey::create_host_key(&handle);
    }
}

#[tauri::command]
fn delete_hot_key(handle: tauri::AppHandle, id: String) {
    println!("111");
    let _ = hot_key::delete_by_id(id);
    #[cfg(all(desktop))]
    {
        if handle.remove_plugin("global-shortcut")  {
            println!("成功global-shortcut");
        } else {
            println!("失败global-shortcut");
        }  

        let _ = hotKey::create_host_key(&handle); 
    }

}


#[tokio::main]
async fn init() {
    //init_mysql_pool("mysql://账号:密码@数据库地址/数据库");
    init_mysql_pool("mydatabase.db").await;
}

fn show_window(app: &AppHandle) {
    match app.get_webview_window("main") {
        Some(win) => {
            let _ = win.show();
        }
        None => {
            let window = app.webview_windows();
            window
                .values()
                .next()
                .expect("Sorry, no window found")
                .set_focus()
                .expect("Can't Bring Window to Focus");
        }
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init();
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            let mainwindow = app.get_webview_window("main").unwrap();
            let _ = mainwindow.show();

            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
                hotKey::create_host_key(handle)?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let label = window.label();
                if label.eq("main") {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            compress_img,
            open_docs,
            savn_cron,
            stop_cron,
            update_cron,
            del_cron,
            get_list_cron,
            use_cron,
            read_file,
            get_cron_info,
            grid_merge_data,
            add_grid,
            update_grid,
            delete_grid_by_id,
            get_grid_by_id,
            floating_window,
            get_tree_cron,
            get_hot_key_list,
            add_hot_key,
            update_hot_key,
            delete_hot_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
