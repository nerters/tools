use std::sync::atomic::{AtomicBool, Ordering};
use dao::{cron::{alert_win, delete_by_id, save, update_tokio, update_use_tokio, CronInfo, CRON_MAP}, grid_info::{self, merge_data, GridInfo}};
use idgen::IDGen;
use rdev::{Button, EventType};
use tauri::Manager;
use tauri::async_runtime::spawn;
use img::img_utils::compress;
use utils::{date_util::get_now_time_m, mysql_utils::init_mysql_pool};
mod img;
mod json;
mod dao;
mod utils;

static FLOATING: AtomicBool = AtomicBool::new(false);

static IS_SIMULATE:bool = false;

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
async fn get_list_cron() -> Vec<CronInfo> {
    let map = CRON_MAP.lock().await;
    map.values().cloned().collect()
}

#[tauri::command]
async fn del_cron(id : String) {
    delete_by_id(id.clone()).await;
    let mut map = CRON_MAP.lock().await;
    map.remove(&id);
}



#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
    alert_win(handle.clone()).await;

    //floating_window(handle.clone()).await;

}

#[tauri::command]
async fn use_cron(handle: tauri::AppHandle, id : String) {
    if !id.is_empty() {
        let mut map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&id) {
            let mut temp = temp.clone();
            temp.update_time = get_now_time_m();
            temp.is_use = 0;
            map.insert(id.clone(), temp.clone());
            if let Some(window) = handle.get_webview_window("main") {
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
    }
}


#[tauri::command]
async fn get_cron_info(id : String) -> CronInfo {
    if !id.is_empty() {
        let map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&id) {
            let temp = temp.clone();
            return temp;    
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
    },
    Err(e) => {
      println!("获取数据失败！{}", e);
      return format!("#@#error#@#_{}", e);
  },
  }
}


#[tauri::command]
async fn grid_merge_data(data_list: Vec<GridInfo>) -> Vec<GridInfo> {
    println!("grid_merge_data");
    merge_data(data_list).await
}


#[tauri::command]
async fn add_grid(name: String, describe: String, uri: String, code: String, classify: String, x: i64, y: i64, w: i64, h: i64) -> i64 {
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
async fn update_grid(id: String, name: String, describe: String, uri: String, code: String, classify: String, x: i64, y: i64, w: i64, h: i64) -> bool {
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
    let mut title = String::from("");
    let map = CRON_MAP.lock().await;
    if let Some(temp) = map.get(&id) {
        let temp = temp.clone();
        win_key += &temp.id.clone();
        title += &temp.name.clone();
    }

    
    if let Some(_win) = handle.get_webview_window(&win_key) {
        println!("窗口已启动!");
    } else {
        let docs_window = tauri::WebviewWindowBuilder::new(
            &handle,
            win_key, /* the unique window label */
            tauri::WebviewUrl::App("/hint/CountDown".parse().unwrap())
          ).title(title)
          .inner_size(300.0, 100.0)
          .decorations(false)
          .transparent(true)
          .position(800.0,100.0)
          .build();
    
        match docs_window {
            Ok(win) => {
                let _ = win.set_always_on_top(true);
            },
            Err(_) => {
                println!("启动窗口失败!");
            }, 
        }
    }
}




//悬浮窗口
async fn floating_window111(handle: tauri::AppHandle){
    spawn(async move {
   
        // 检查标志位，如果线程已经启动，则不再执行
        if FLOATING.load(Ordering::SeqCst) {
            println!("悬浮窗口线程已启动");
            return;
        }
      
        // 设置标志位，表示线程已经启动
        FLOATING.store(true, Ordering::SeqCst);
        rdev::listen(move |event| {
            let is_block: bool = match event.event_type {
                EventType::ButtonPress(button) => {
                    match button {
                        Button::Right => {
                            println!("right");
                             unsafe {
                                !IS_SIMULATE
                             }
                        }
                        _ => { false }
                    }
                }
                EventType::ButtonRelease(button) => {
                    match button {
                        Button::Right => {
                            println!("right");
                            unsafe {
                                !IS_SIMULATE
                            }
                        }
                        _ => { false }
                    }
                }
                
                _ => { false }
            };
            if is_block {
                let docs_window = tauri::WebviewWindowBuilder::new(
                    &handle,
                    "floating-111", /* the unique window label */
                    tauri::WebviewUrl::App("/hint/Ment".parse().unwrap())
                  ).title("test")
                  .inner_size(100.0, 100.0)
                  .decorations(false)
                  .transparent(true)
                  .position(0.0,200.0)
                  .build();
    
                match docs_window {
                    Ok(win) => {
                        //win.emit("buttondown", ButtonPayload { button: get_button_name(&button), x: MOUSE_POSITION.0, y: MOUSE_POSITION.1 }).unwrap();
    
                        win.set_always_on_top(true);
                        println!("启动窗口成功!");
                    },
                    Err(e) => {
                        println!("启动窗口失败!{}", e);
    
                    }, 
                }
            }
            // if is_block {
            //     None
            // } else {
            //     Some(event)
            // }
        }).unwrap();
        println!("悬浮窗口线程已结束");
        // 设置标志位，表示线程已经启动
        FLOATING.store(false, Ordering::SeqCst);
    });

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
        .invoke_handler(tauri::generate_handler![compress_img, open_docs,savn_cron, update_cron, 
            del_cron, get_list_cron, use_cron, read_file, get_cron_info, grid_merge_data, add_grid, 
            update_grid, delete_grid_by_id, get_grid_by_id, floating_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");



   
}
