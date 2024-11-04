use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::utils::{date_util::get_now_time_m, mysql_utils::get_connect};

lazy_static! {
    pub static ref CRON_MAP: Arc<Mutex<HashMap<String, HotKey>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HotKey {
    pub id: String,
    pub key: String,
    pub path: String,
    pub desc: String,
    pub overopen: i64,
    pub url: String,
    pub shell: String,

    pub key1: String,
    pub key2: String,

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
}

impl Default for HotKey {
    fn default() -> Self {
        HotKey {
            id: String::from(""),
            key: String::from(""),
            path: String::from(""),
            key1: String::from(""),
            key2: String::from(""),
            desc: String::from(""),
            overopen: 0,
            url: String::from(""),
            shell: String::from(""),

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
        }
    }
}

impl HotKey {

}

#[tokio::main]
pub async fn get_list() -> Vec<HotKey> {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, key, path, desc, create_time, update_time, overopen, url, shell from hot_key")
    .map(|row: SqliteRow| {
        HotKey{ id: row.get(0), key: row.get(1), path: row.get(2), key1: String::from(""), key2: String::from(""), desc: row.get(3), create_time: row.get(4), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, 
            update_time: row.get(5), overopen: row.get(6), url: row.get(7), shell: row.get(8)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(mut rest) => {
            for ele in &mut rest {
                let parts: Vec<&str> = ele.key.split('+').collect();
                if parts.len() == 2 {
                    ele.key1 = parts[0].to_owned();
                    ele.key2 = parts[1].to_string();
                } else if parts.len() == 3 {
                    ele.key1 = parts[0].to_owned() + "+" + parts[1];
                    ele.key2 = parts[2].to_string();
                } else if parts.len() == 4 {
                    ele.key1 = parts[0].to_owned() + "+" + parts[1] + "+" + parts[2];
                    ele.key2 = parts[3].to_string();
                }
            }
            return rest;
        }
        Err(e) => {
            println!("失败{}", e);
            return vec![];
        }
    }
}

#[tokio::main]
pub async fn get_list_tokio() -> Vec<HotKey> {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, key, path, desc, create_time, update_time, overopen, url, shell from hot_key")
    .map(|row: SqliteRow| {
        HotKey{ id: row.get(0), key: row.get(1), path: row.get(2), key1: String::from(""), key2: String::from(""), desc: row.get(3), create_time: row.get(4), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, 
            update_time: row.get(5), overopen: row.get(6), url: row.get(7), shell: row.get(8)}
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

#[tokio::main]
pub async fn save(info: HotKey) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let now = get_now_time_m();
    let result = sqlx::query("insert into hot_key (id, key, path, desc, create_time, update_time, deleted, overopen, url, shell) values (?,?,?,?,?,?,?,?,?,?)")
    .bind(info.id)
    .bind(info.key)
    .bind(info.path)
    .bind(info.desc)
    .bind(now).bind(now).bind(0).bind(info.overopen)
    .bind(info.url)
    .bind(info.shell)
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
pub async fn update_tokio(info: HotKey) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE hot_key SET key = ?,path=?, desc=?,update_time=?,sort=?,overopen=?, url = ?, shell = ?  where id = ?")
    .bind(info.key)
    .bind(info.path)
    .bind(info.desc)
    .bind(get_now_time_m()).bind(info.sort)
    .bind(info.overopen)
    .bind(info.url)
    .bind(info.shell)
    .bind(info.id)
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
pub async fn update(info: HotKey) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE hot_key SET key = ?,path=?, desc=?,update_time=?,sort=?,overopen=?, url = ?, shell = ?  where id = ?")
    .bind(info.key)
    .bind(info.path)
    .bind(info.desc)
    .bind(get_now_time_m()).bind(info.sort)
    .bind(info.overopen)
    .bind(info.url)
    .bind(info.shell)
    .bind(info.id)
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
pub async fn delete_by_id(id: String) -> bool {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    println!("{}", id);
    let result = sqlx::query("delete from hot_key where id = ?")
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

#[tokio::main]
pub async fn get_info_by_key(key: String) -> HotKey {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, key, path, desc, create_time, update_time,overopen, url,shell from hot_key where key = ?")
    .bind(key)
    .map(|row: SqliteRow| {
        HotKey{ id: row.get(0), key: row.get(1), path: row.get(2), key1: String::from(""), key2: String::from(""), desc: row.get(3), create_time: row.get(4), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, 
            update_time: row.get(5), overopen: row.get(6), url: row.get(7), shell: row.get(8)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(rest) => {
            if let Some(res) = rest.get(0) {
                return res.clone();
            }
            return HotKey::default();
        }
        Err(e) => {
            println!("失败{}", e);
            return HotKey::default();
        }
    }
}


#[tokio::main]
pub async fn get_info_by_id(id: String) -> HotKey {
    let conn = get_connect()
        .acquire()
        .await
        .expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, key, path, desc, create_time, update_time,overopen, url, shell from hot_key where id = ?")
    .bind(id)
    .map(|row: SqliteRow| {
        HotKey{ id: row.get(0), key: row.get(1), path: row.get(2), key1: String::from(""), key2: String::from(""), desc: row.get(3), create_time: row.get(4), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, 
            update_time: row.get(5), overopen: row.get(6), url: row.get(7), shell: row.get(8)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(rest) => {
            if let Some(res) = rest.get(0) {
                return res.clone();
            }
            return HotKey::default();
        }
        Err(e) => {
            println!("失败{}", e);
            return HotKey::default();
        }
    }
}