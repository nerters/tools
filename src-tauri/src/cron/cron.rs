use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row};

use crate::utils::{date_util::get_now_time_m, mysql_utils::get_connect};


#[derive(Serialize, Deserialize, Clone)]
pub struct CronInfo {
    pub id:String,
    pub name:String,
    pub content:String,
    pub interval:i64,
    pub appointed_time:i64,
    pub is_use:i64,
    pub create_time:i64,
    pub creator_lid:String,
    pub creator_name:String,
    pub updater_lid:String,
    pub updater_name:String,
    pub up_ver:i16,
    pub sort:i16,
    pub tenant_id:i64,
    pub deleted:i64,
    pub update_time:i64,
}

impl Default for CronInfo {
    fn default() -> Self {
        CronInfo {
            id:String::from(""),
            name: String::from(""),
            content:String::from(""),
            interval: 0,
            appointed_time: 0,
            is_use: 0,
            create_time: 0,
            creator_lid: String::from(""),
            creator_name: String::from(""),
            updater_lid: String::from(""),
            updater_name: String::from(""),
            up_ver: 0,
            sort: 0,
            tenant_id: 0,
            deleted: 0,
            update_time: 0
        }
    }
}

pub async fn getList() -> Vec<CronInfo> {
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, name, content, interval, appointed_time, create_time, update_time, is_use from cron_title")
    .map(|row: SqliteRow| {
        CronInfo{ id: row.get(0), name: row.get(1), content: row.get(2), interval: row.get(3), appointed_time: row.get(4), create_time: row.get(5), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, update_time: row.get(6), is_use:row.get(7)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(rest) => {
            return rest;
        },
        Err(e) => {
            println!("失败{}", e);
            return vec![];
        },
    }
}

#[tokio::main]
pub async fn get_list_tokio() -> Vec<CronInfo> {
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, name, content, interval, appointed_time, create_time, update_time, is_use from cron_title")
    .map(|row: SqliteRow| {
        CronInfo{ id: row.get(0), name: row.get(1), content: row.get(2), interval: row.get(3), appointed_time: row.get(4), create_time: row.get(5), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, update_time: row.get(6), is_use:row.get(7) }
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(rest) => {
            return rest;
        },
        Err(e) => {
            println!("失败{}", e);
            return vec![];
        },
    }
}

#[tokio::main]
pub async fn save(info: CronInfo) -> bool {
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let now = get_now_time_m();
    let result = sqlx::query("insert into cron_title (id, name, content, interval, appointed_time, is_use, create_time, update_time, deleted) values (?,?,?,?,?,?,?,?,?)")
    .bind(info.id).bind(info.name).bind(info.content).bind(info.interval).bind(info.appointed_time).bind(0).bind(now).bind(now).bind(0)
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
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE cron_title SET content = ?,interval=?,appointed_time=?,update_time=?,is_use=?  where id = ?")
    .bind(info.content).bind(info.interval).bind(info.appointed_time).bind(info.update_time).bind(info.is_use).bind(info.id)
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
pub async fn update_use_tokio(info: CronInfo) -> bool {
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE cron_title SET update_time=?,is_use=?  where id = ?")
    .bind(info.update_time).bind(info.is_use).bind(info.id)
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

pub async fn update(info: CronInfo) -> bool {
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE cron_title SET content = ?,interval=?,appointed_time=?,update_time=?,is_use=?  where id = ?")
    .bind(info.content).bind(info.interval).bind(info.appointed_time).bind(info.update_time).bind(info.is_use).bind(info.id)
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
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    println!("{}",id);
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