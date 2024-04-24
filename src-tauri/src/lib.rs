use core::time;
use std::{collections::HashMap, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread::{self, sleep}};
use cron::cron::{delete_by_id, getList, get_list_tokio, save, update, update_tokio, update_use_tokio, CronInfo};
use idgen::IDGen;
use tauri::{ Manager};
use tauri::async_runtime::spawn;
use img::img_utils::compress;
use serde_json::{Error, Value};
use utils::{date_util::get_now_time_m, mysql_utils::init_mysql_pool};
mod img;
mod json;
mod cron;
mod utils;


static THREAD_STARTED: AtomicBool = AtomicBool::new(false);


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn json_diff(old_json: String, new_json: String) -> HashMap<String, String> {
    let mut rest = HashMap::new();
    let old: Result<Value, Error> = serde_json::from_str(&old_json);
    let mut old_text = Value::default();
    match old {
        Ok(text) => old_text = text,
        Err(_) => {
            println!("解析old错误！");
        }
    }
    let mut new_text = Value::default();
    let new: Result<Value, Error> = serde_json::from_str(&new_json);
    match new {
        Ok(text) => new_text = text,
        Err(_) => {
            println!("解析new错误！");
        }
    }

    let (old_res, new_res) = json::json_utils::json_diff(&old_text, &new_text);
    rest.insert(
        "old".to_string(),
        match serde_json::to_string_pretty(&old_res) {
            Ok(data) => data
                .replace("  ", "&emsp;&emsp;&emsp;")
                .replace("\n", "<br>")
                .replace("\\\"", "\""),
            Err(_) => "".to_string(),
        },
    );
    rest.insert(
        "new".to_string(),
        match serde_json::to_string_pretty(&new_res) {
            Ok(data) => data
                .replace("  ", "&emsp;&emsp;&emsp;")
                .replace("\n", "<br>")
                .replace("\\\"", "\""),
            Err(_) => "".to_string(),
        },
    );
    rest
}

#[tauri::command]
fn compress_img(file_path: String, nwidth: u32, nheight: u32, img_type: String) -> String {
    compress(&file_path, nwidth, nheight, img_type)
}

#[tauri::command]
fn savn_cron(name: String, content: String, interval: i64, appointed_time: i64) -> i64 {
    let idgen = IDGen::new(1);
    let id = idgen.new_id();
    let mut info = CronInfo::default();
    info.id = id.to_string();
    info.name = name;
    info.content = content;
    info.interval = interval;
    info.appointed_time = appointed_time;
    save(info);
    id as i64
}

#[tauri::command]
fn update_cron(id: String, name: String, content: String, interval: i64, appointed_time: i64) {
    let mut info = CronInfo::default();
    info.id = id;
    info.name = name;
    info.content = content;
    info.interval = interval;
    info.appointed_time = appointed_time;
    update_tokio(info);
}

#[tauri::command]
fn get_list_cron() -> Vec<CronInfo> {
    get_list_tokio()
}

#[tauri::command]
fn del_cron(id : String) {
    delete_by_id(id);
}



#[tauri::command]
fn open_docs(handle: tauri::AppHandle) {
    spawn(async move {
        // 检查标志位，如果线程已经启动，则不再执行
        if THREAD_STARTED.load(Ordering::SeqCst) {
            return;
        }
        thread_funtion(handle).await;
    });
}

#[tauri::command]
fn use_cron(id : String) {
    if !id.is_empty() {
        let mut info = CronInfo::default();
        info.id = id;
        info.is_use = 0;
        info.update_time = get_now_time_m();
        update_use_tokio(info);
    }
}



async fn thread_funtion(handle: tauri::AppHandle) {

    
        // 设置标志位，表示线程已经启动
        THREAD_STARTED.store(true, Ordering::SeqCst);
        // 在新线程中执行的代码
        loop {
            println!("123");
            let now = get_now_time_m();
            let list = getList().await;
            for i in list.iter() {
                if i.is_use == 0 && i.interval + i.update_time < now {

                    let docs_window = tauri::WebviewWindowBuilder::new(
                        &handle,
                        "dev-".to_string() + i.id.to_string().as_mut_str(), /* the unique window label */
                        tauri::WebviewUrl::App("/hint/Cron".parse().unwrap())
                      ).title(i.content.clone() + "_+||+_" + i.id.to_string().as_str())
                      .inner_size(300.0, 100.0)
                      .decorations(false)
                      .transparent(true)
                      .position(0.0,400.0)

                      .build();
                    match docs_window {
                        Ok(win) => {
                            win.set_always_on_top(true);
                           
                        },
                        Err(_) => {
                            println!("启动窗口失败!");
       
                        }, 
                    }
                    let mut temp = i.clone();
                    temp.update_time = get_now_time_m();
                    update(temp).await;
                }
            }
        
            let ten_millis = time::Duration::from_millis(1000 * 5);
            sleep(ten_millis)
        }
        // 设置标志位，表示线程已经启动
        THREAD_STARTED.store(false, Ordering::SeqCst);
}


#[tokio::main]
async fn init() {
  //init_mysql_pool("mysql://账号:密码@数据库地址/数据库");
  init_mysql_pool("mydatabase.db").await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    
    init();
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app|{
            let mainwindow = app.get_webview_window("main").unwrap();
            let _ = mainwindow.show();
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![json_diff, compress_img, open_docs,savn_cron, update_cron, del_cron, get_list_cron, use_cron])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
