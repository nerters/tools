use std::collections::HashMap;

use ollama_rs::Ollama;
use tauri::{AppHandle, Manager};
use once_cell::sync::OnceCell;
use utils::mysql_utils::init_mysql_pool;
mod dao;
mod img;
mod json;
mod utils;
mod service;

#[cfg(desktop)]
mod tray;
#[cfg(desktop)]
mod hotKey;

pub static ALLAMA: OnceCell<Ollama> = OnceCell::new();
pub static PHYSIZE: OnceCell<HashMap<String, u32>> = OnceCell::new();
pub static mut SCALE_FACTOR : f64 = 0.0;



#[tokio::main]
async fn init() {
    //init_mysql_pool("mysql://账号:密码@数据库地址/数据库");
    init_mysql_pool("mydatabase.db").await;
       // For custom values:
    ALLAMA
    .set(Ollama::new("http://localhost".to_string(), 11434))
    .unwrap_or_else(|_| println!("try insert pool cell failure!"));
    
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

            // 获取主窗口所在的屏幕
            if let Some(monitor) = mainwindow.primary_monitor().unwrap() {
                
                let size = monitor.size();
                let width = size.width;
                let height = size.height;
                unsafe { SCALE_FACTOR = monitor.scale_factor() };
                let mut map: HashMap<String, u32> = HashMap::new();
                map.insert(String::from("width"), width);
                map.insert(String::from("height"), height);

                println!("scale_factor: {}", monitor.scale_factor());

                PHYSIZE
                .set(map)
                .unwrap_or_else(|_| println!("try insert pool cell failure!"));
              
                println!("Screen resolution: {}x{}", width, height);
            } else {
                println!("Could not get monitor information");
            }

            let _ = mainwindow.show();

            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
                hotKey::create_host_key(handle)?;


                //自动启动
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                let _ = app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec!["--flag1", "--flag2"]),
                ));

                // Get the autostart manager
                let autostart_manager = app.autolaunch();
                // Enable autostart
                let _ = autostart_manager.enable();
                // Check enable state
                println!("registered for autostart? {}", autostart_manager.is_enabled().unwrap());
                // Disable autostart
                let _ = autostart_manager.disable();

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
            service::img_serve::compress_img,
            service::cron_serve::open_docs,
            service::cron_serve::savn_cron,
            service::cron_serve::stop_cron,
            service::cron_serve::update_cron,
            service::cron_serve::del_cron,
            service::cron_serve::get_list_cron,
            service::cron_serve::use_cron,
            service::cron_serve::get_cron_info,
            service::cron_serve::get_tree_cron,
            service::cron_serve::floating_window,

            service::file_serve::read_file,
            
            service::grid_serve::grid_merge_data,
            service::grid_serve::add_grid,
            service::grid_serve::update_grid,
            service::grid_serve::delete_grid_by_id,
            service::grid_serve::get_grid_by_id,
           
            service::hot_key_serve::get_hot_key_list,
            service::hot_key_serve::add_hot_key,
            service::hot_key_serve::update_hot_key,
            service::hot_key_serve::delete_hot_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
