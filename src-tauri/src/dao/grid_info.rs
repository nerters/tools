use std::collections::HashMap;

use idgen::IDGen;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row};

use crate::utils::{date_util::get_now_time_m, mysql_utils::get_connect};

#[derive(Serialize, Deserialize, Clone)]
pub struct GridInfo {
    pub id:String,
    pub name:String,
    pub describe:String,
    pub uri:String,
    pub code:String,
    pub classify:String,
    pub is_sys:i64,
    pub x:i64,
    pub y:i64,
    pub w:i64,
    pub h:i64,
    pub template_id: String,
    pub run_code: String,



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

impl Default for GridInfo {
    fn default() -> Self {
        GridInfo {
            id:String::from(""),
            name: String::from(""),
            describe:String::from(""),
            uri: String::from(""),
            code: String::from(""),
            classify: String::from(""),
            is_sys: 0,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            template_id: String::from(""),
            run_code: String::from(""),


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


pub async fn get_list() -> Vec<GridInfo> {
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, name, describe, uri, code, classify,create_time, update_time, is_sys,x,y,w,h,template_id,run_code from grid_info")
    .map(|row: SqliteRow| {
        GridInfo{ id: row.get(0), name: row.get(1), describe: row.get(2), uri: row.get(3), code: row.get(4), classify: row.get(5), create_time: row.get(6), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, update_time: row.get(7),
             is_sys: row.get(8), x: row.get(9), y: row.get(10) , w: row.get(11), h: row.get(12), template_id: row.get(13), run_code: row.get(14)}
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


pub async fn get_info_by_id(id: String) -> GridInfo {
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let rust = sqlx::query("select id, name, describe, uri, code, classify,create_time, update_time, is_sys, x, y, w, h,template_id,run_code from grid_info where id = ?")
    .bind(id)
    .map(|row: SqliteRow| {
        GridInfo{ id: row.get(0), name: row.get(1), describe: row.get(2), uri: row.get(3), code: row.get(4), classify: row.get(5), create_time: row.get(6), 
            creator_lid: String::from(""), creator_name: String::from(""), updater_lid: String::from(""), updater_name: String::from(""), up_ver: 0, sort: 0, tenant_id: 0, deleted: 0, update_time: row.get(7),
             is_sys: row.get(8), x: row.get(9), y: row.get(10) , w: row.get(11), h: row.get(12), template_id: row.get(13), run_code: row.get(14)}
    })
    .fetch_all(&mut conn.detach())
    .await;

    match rust {
        Ok(rest) => {
            if let Some(res) = rest.get(0) {
                return res.clone();
            }
            return GridInfo::default();
        },
        Err(e) => {
            println!("失败{}", e);
            return GridInfo::default();
        },
    }
}



pub async fn save(info: GridInfo) -> bool {
    let mut w = info.w;
    let mut h = info.h;
    if w <= 0 {
        w = 2;
    }
    if h <= 0 {
        h = 2;
    }
    println!("x : {}", info.x);
    println!("y : {}", info.y);
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let now = get_now_time_m();
    let result = sqlx::query("insert into grid_info (id, name, describe, uri, code, classify, is_sys, x, y, w, h, template_id, run_code, 
        create_time, update_time, deleted) values (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")
    .bind(info.id).bind(info.name).bind(info.describe).bind(info.uri).bind(info.code).bind(info.classify)
    .bind(info.is_sys).bind(info.x).bind(info.y).bind(w).bind(h)
    .bind(info.template_id).bind(info.run_code).bind(now).bind(now).bind(0)
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


pub async fn save_list(list: Vec<GridInfo>) ->i32 {
    let mut f_num = 0;
    for info in list.iter() {
        if !save(info.clone()).await {
            f_num += 1;
        }
    }
    if f_num == 0 {
        println!("全部保存成功");
        return 1;
    }
    if f_num == list.len() {
        println!("保存失败");
        return -1;
    }
    println!("部分数据保存失败");
    return 0;
}




pub async fn update(info: GridInfo) -> bool {
    let mut w = info.w;
    let mut h = info.h;
    if w <= 0 {
        w = 2;
    }
    if h <= 0 {
        h = 2;
    }
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    let result = sqlx::query("UPDATE grid_info SET name = ?, describe = ?,uri=?, code=?, update_time=?, classify=?, 
    is_sys=?, x=?, y=?, w=?, h=?,
    template_id=?, run_code=?  where id = ?")
    .bind(info.name).bind(info.describe).bind(info.uri).bind(info.code).bind(info.update_time).bind(info.classify)
    .bind(info.is_sys).bind(info.x).bind(info.y).bind(w).bind(h)
    .bind(info.template_id).bind(info.run_code).bind(info.id)
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
    let conn = get_connect().acquire().await.expect("Error get_connect from db pool");
    println!("{}",id);
    let result = sqlx::query("delete from grid_info where id = ?")
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



pub async fn merge_data(data_list: Vec<GridInfo>) -> Vec<GridInfo> {
    let list = get_list().await;
    if list.is_empty() {
        let state = save_list(data_list.clone()).await;
        return data_list;
    } else {
        let mut map: HashMap<String, GridInfo> = HashMap::new();
        for mut info in data_list {
            if info.id.is_empty() || info.id == "" {
                let idgen = IDGen::new(1);
                let id = idgen.new_id();
                info.id = id.to_string();
                save(info).await;
            } else {
                map.insert(info.id.clone(), info);
            }
            
        }

        for info in list {
            if let Some(grid) = map.remove(&info.id) {
                update(grid.clone()).await;
            }
        }
        let temp: Vec<GridInfo> = map.values().cloned().collect();
        for info in temp {
            save(info).await;
        }
    }
    get_list().await
}