use core::time;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
};
use tauri::{async_runtime::spawn, Manager, PhysicalPosition};
use tokio::sync::Mutex;

use crate::{utils::{date_util::get_now_time_m, mysql_utils::get_connect}, PHYSIZE};

static THREAD_STARTED: AtomicBool = AtomicBool::new(false);

pub static CRON_UPDATE: AtomicBool = AtomicBool::new(false);

lazy_static! {
    pub static ref CRON_MAP: Arc<Mutex<HashMap<String, CronInfo>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CronInfo {
    pub id: String,
    pub name: String,
    pub content: String,
    pub cron_type: String,
    pub interval: i64,
    pub appointed_time: i64,
    pub is_use: i64,
    pub pid: String,
    pub category: String,
    pub activity: i64,

    pub create_time: i64,
    pub creator_lid: String,
    pub creator_name: String,
    pub updater_lid: String,
    pub updater_name: String,
    pub up_ver: i16,
    pub sort: i16,
    pub tenant_id: i64,
    pub deleted: i64,
    pub update_time: i64,

    pub children: Vec<CronInfo>,
}

impl Default for CronInfo {
    fn default() -> Self {
        CronInfo {
            id: String::from(""),
            name: String::from(""),
            content: String::from(""),
            cron_type: String::from(""),
            interval: 0,
            appointed_time: 0,
            is_use: 0,
            pid: String::from(""),
            category: String::from(""),
            activity: 1,

            create_time: 0,
            creator_lid: String::from(""),
            creator_name: String::from(""),
            updater_lid: String::from(""),
            updater_name: String::from(""),
            up_ver: 0,
            sort: 0,
            tenant_id: 0,
            deleted: 0,
            update_time: 0,
            children: vec![],
        }
    }
}

impl CronInfo {
    fn set_children(mut self: Self, children: Vec<CronInfo>) {
        self.children = children;
    }
}

pub async fn get_list() -> Vec<CronInfo> {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, name, content, interval, appointed_time, create_time, update_time, is_use, cron_type, pid, category, activity from cron_title")
    .map(|row: SqliteRow| {
        CronInfo{ id: row.get(0), name: row.get(1), content: row.get(2), interval: row.get(3), appointed_time: row.get(4), create_time: row.get(5), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, 
            update_time: row.get(6), is_use:row.get(7), cron_type:row.get(8), pid:row.get(9), category:row.get(10), children:vec![], activity: row.get(11)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(rest) => {
            return rest;
        }
        Err(e) => {
            println!("失败{}", e);
            return vec![];
        }
    }
}

pub async fn get_list_by_pid_and_category(pid: String, category: String) -> Vec<CronInfo> {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let mut params: Vec<String> = vec![];
    if !pid.is_empty() {
        params.push(" pid = '".to_owned() + &pid + "'")
    }
    if !category.is_empty() {
        params.push(" category = '".to_owned() + &category + "'")
    }
    let mut param = String::from("");
    if !params.is_empty() {
        param += &("where ".to_string() + &params.join(" and "));
    }
    let type_list = sqlx::query(&("select id, name, content, interval, appointed_time, create_time, update_time, is_use, cron_type, pid, category, activity from cron_title ".to_string() + &param))
    .map(|row: SqliteRow| {
        CronInfo{ id: row.get(0), name: row.get(1), content: row.get(2), interval: row.get(3), appointed_time: row.get(4), create_time: row.get(5), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, 
            update_time: row.get(6), is_use:row.get(7), cron_type:row.get(8), pid:row.get(9), category:row.get(10), children:vec![], activity: row.get(11)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match type_list {
        Ok(rest) => {
            return rest;
        }
        Err(e) => {
            println!("get_list_by_pid_and_category 失败:{}", e);
            return vec![];
        }
    }
}

pub async fn get_tree() -> Vec<CronInfo> {
    let mut type_list =
        get_list_by_pid_and_category(String::from("-2"), String::from("type")).await;
    println!("查询cron type -2的数量:{}", type_list.len());
    for i in type_list.iter_mut() {
        let id = i.id.clone();
        let mut list = get_list_by_pid_and_category(id, String::from("type")).await;
        for c in list.iter_mut() {
            let id = c.id.clone();
            c.children = get_list_by_pid_and_category(id, String::from("cron")).await;
        }
        i.children = list;
    }

    type_list
}

#[tokio::main]
pub async fn get_list_tokio() -> Vec<CronInfo> {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, name, content, interval, appointed_time, create_time, update_time, is_use, cron_type, pid, category, activity from cron_title")
    .map(|row: SqliteRow| {
        CronInfo{ id: row.get(0), name: row.get(1), content: row.get(2), interval: row.get(3), appointed_time: row.get(4), create_time: row.get(5), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, update_time: row.get(6),
             is_use:row.get(7), cron_type:row.get(8), pid:row.get(9), category:row.get(10), children:vec![], activity: row.get(11)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(rest) => {
            return rest;
        }
        Err(e) => {
            println!("失败{}", e);
            return vec![];
        }
    }
}

pub async fn save(info: CronInfo) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let now = get_now_time_m();
    let result = sqlx::query("insert into cron_title (id, name, content, interval, cron_type, appointed_time, is_use, pid, category, create_time, update_time, deleted, activity) values (?,?,?,?,?,?,?,?,?,?,?,?,?)")
    .bind(info.id).bind(info.name).bind(info.content).bind(info.interval).bind(info.cron_type).bind(info.appointed_time).bind(info.is_use).bind(info.pid)
    .bind(info.category).bind(now).bind(now).bind(0).bind(info.activity)
    .execute(&mut conn.detach())
    .await;

    match result {
        Ok(_) => {
            println!("创建数据成功");
            return true;
        }
        Err(e) => {
            println!("{}", e);
            println!("创建数据失败");
            return false;
        }
    }
}

#[tokio::main]
pub async fn update_tokio(info: CronInfo) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE cron_title SET content = ?,interval=?, cron_type=?,appointed_time=?,update_time=?,is_use=?,pid=?,category=?, sort=?, activity=?  where id = ?")
    .bind(info.content).bind(info.interval).bind(info.cron_type).bind(info.appointed_time).bind(get_now_time_m()).bind(info.is_use).bind(info.pid).bind(info.category).bind(info.sort).bind(info.activity).bind(info.id)
    .execute(&mut conn.detach())
    .await;
    CRON_UPDATE.store(true, Ordering::SeqCst);

    match result {
        Ok(_) => {
            println!("创建数据成功");
            return true;
        }
        Err(e) => {
            println!("{}", e);
            println!("创建数据失败");
            return false;
        }
    }
}

pub async fn update_use_tokio(info: CronInfo) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE cron_title SET update_time=?,is_use=?  where id = ?")
        .bind(info.update_time)
        .bind(info.is_use)
        .bind(info.id)
        .execute(&mut conn.detach())
        .await;

    match result {
        Ok(_) => {
            println!("cron保存 {} 成功", info.name);
            return true;
        }
        Err(e) => {
            println!("{}", e);
            println!("创建数据失败");
            return false;
        }
    }
}


pub async fn stop_cron(info: CronInfo) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE cron_title SET update_time=?,activity=?,is_use=?  where id = ?")
        .bind(info.update_time)
        .bind(info.activity)
        .bind(0)
        .bind(info.id)
        .execute(&mut conn.detach())
        .await;

    match result {
        Ok(_) => {
            println!("cron保存 {} 成功", info.name);
            return true;
        }
        Err(e) => {
            println!("{}", e);
            println!("创建数据失败");
            return false;
        }
    }
}

pub async fn update(info: CronInfo) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE cron_title SET content = ?,interval=?, cron_type=?,appointed_time=?,update_time=?,is_use=?,pid=?,category=?, activity=?  where id = ?")
    .bind(info.content).bind(info.interval).bind(info.cron_type).bind(info.appointed_time).bind(info.update_time).bind(info.is_use).bind(info.pid).bind(info.category).bind(info.activity).bind(info.id)
    .execute(&mut conn.detach())
    .await;

    match result {
        Ok(_) => {
            println!("创建数据成功");
            return true;
        }
        Err(e) => {
            println!("{}", e);
            println!("创建数据失败");
            return false;
        }
    }
}

pub async fn delete_by_id(id: String) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    println!("{}", id);
    let result = sqlx::query("delete from cron_title where id = ?")
        .bind(id)
        .execute(&mut conn.detach())
        .await;

    match result {
        Ok(_) => {
            println!("创建数据成功");
            return true;
        }
        Err(e) => {
            println!("{}", e);
            println!("创建数据失败");
            return false;
        }
    }
}

pub async fn get_list_cache() -> Vec<CronInfo> {
    //let mut list = CRON_LIST.lock().await;
    let mut map = CRON_MAP.lock().await;
    if map.len() == 0 || CRON_UPDATE.load(Ordering::SeqCst) {
        let cron_list = get_list_by_pid_and_category(String::from(""), String::from("cron")).await;
        for i in cron_list.iter() {
            map.insert(i.id.clone(), i.clone());
        }
        CRON_UPDATE.store(false, Ordering::SeqCst);
    }
    map.values().cloned().collect()
}

//提醒窗口
pub async fn alert_win(handle: tauri::AppHandle) {
    spawn(async move {
        // 检查标志位，如果线程已经启动，则不再执行
        if THREAD_STARTED.load(Ordering::SeqCst) {
            println!("线程已启动");
            return;
        }
        // 设置标志位，表示线程已经启动
        THREAD_STARTED.store(true, Ordering::SeqCst);

        // 在新线程中执行的代码
        loop {
            let now = get_now_time_m();
            //获取缓存中的值
            let list = get_list_cache().await;
            for i in 0..list.len() {
                let list = get_list_cache().await;
                let i = list.get(i);
                match i {
                    Some(i) => {
                        if i.activity == 1 && (i.is_use == 1
                            || (i.cron_type == "interval"
                                && i.is_use == 0
                                && i.interval + i.update_time < now)
                            || (i.cron_type == "appointedTime" && i.is_use == 0 && i.appointed_time < now))
                        {
                            new_win(handle.clone(), i.clone()).await;
                        }
                    },
                    None => {
                        break;
                    },
                }
                let ten_millis = time::Duration::from_millis(1000 * 1);
                sleep(ten_millis)
            }
            let ten_millis = time::Duration::from_millis(1000 * 10);
            sleep(ten_millis)
        }
        // 设置标志位，表示线程已经启动
        THREAD_STARTED.store(false, Ordering::SeqCst);
    });
}

async fn new_win(handle: tauri::AppHandle, cron: CronInfo) {
    let win_key = "dev-".to_string() + &(cron.id.clone());
    if let Some(_win) = handle.get_webview_window(&win_key) {
        println!("{} 窗口已启动!", &(cron.id.clone()));
    } else {
        let win_list = handle.webview_windows();
        let mut win_num = 0;
        for (key, _value) in win_list {
            if key.starts_with("dev") {
                win_num += 1;
            }
        }
        
        let physize = PHYSIZE.get().expect("Error get pool from OneCell<Pool>");
        let width = physize.get("width").unwrap_or(&(1920 as u32)).clone() as f64;
        let height = physize.get("height").unwrap_or(&(1080 as u32)).clone() as f64;

        let position_x = 0.0;
        let mut position_y = height / 2.0 + (win_num as f64) * 100.0;
        if position_y > height {
            position_y = height;
        }

        println!("窗口title {}", cron.id);
        let docs_window = tauri::WebviewWindowBuilder::new(
            &handle,
            win_key, /* the unique window label */
            tauri::WebviewUrl::App("/hint/Alarm".parse().unwrap()),
        )
        .title(cron.id.as_str())
        .inner_size(300.0, 100.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .position(position_x, position_y)
        .build();
        println!("Screen resolution: {}x{}", width, height);
        println!("position: x : {} y : {}", position_x, position_y);
        match docs_window {
            Ok(win) => {
                let _ = win.set_always_on_top(true);
                // 获取主窗口所在的屏幕
                if let Some(monitor) = win.primary_monitor().unwrap() {
                    let size = monitor.size();
                    let width = size.width as f64;
                    let height = size.height as f64;

                    let position_x = 0.0;
                    let mut position_y = height / 2.0 + (win_num as f64) * 100.0;
                    if position_y > height {
                        position_y = height;
                    }
                    let _ = win.set_position(PhysicalPosition::new(position_x, position_y));
            
                    println!("Screen resolution: {}x{}", width, height);
                    println!("position: x : {} y : {}", position_x, position_y);
                } else {
                    println!("Could not get monitor information");
                }
            }
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }

    if cron.is_use == 0 {
        let mut map = CRON_MAP.lock().await;
        if let Some(temp) = map.get(&cron.id) {
            let mut temp = temp.clone();
            temp.is_use = 1;
            temp.update_time = get_now_time_m();
            println!("重置更新时间：{}", temp.update_time);
            map.insert(cron.id.clone(), temp);
        }
    }
}
