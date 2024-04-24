use std::io::Read;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};



// 创建一个全局的DB_POOL，可以一直使用，启动的时候初始化即可
static DB_POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

// 初始化数据库链接池

pub async fn init_mysql_pool(db_url: &str) {
    println!("初始化数据库线程池--------开始-------");
    let path = "C:\\Users\\tjw1t\\AppData\\Roaming\\tk.tools";
    
    if let Ok(metadata) = std::fs::metadata(path.to_string() + "\\" + db_url) {
        if !metadata.is_file() {
            std::fs::create_dir_all(path);
            std::fs::File::create(path.to_string() + "\\" + db_url);
        }
    } else {
        std::fs::create_dir_all(path);
        std::fs::File::create(path.to_string() + "\\" + db_url);
    }


    let pool = SqlitePoolOptions::new().connect_lazy(&("sqlite:".to_string() + path + "\\" + db_url)).ok().expect("链接失败");

    // 执行一个 SQL 查询，例如创建一个表
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS 'cron_title' (
        'id' TEXT NOT NULL,
        'name' TEXT,
        'content' TEXT,
        'interval' INTEGER,
        'appointed_time' INTEGER,
        'is_use' INTEGER,
        'create_time' integer,
        'creator_lid' TEXT,
        'creator_name' TEXT,
        'updater_lid' TEXT,
        'updater_name' TEXT,
        'up_ver' integer,
        'sort' integer,
        'tenant_id' integer,
        'deleted' integer,
        'update_time' integer,
        PRIMARY KEY ('id')
      );")
        .execute(&pool)
        .await;


    DB_POOL.set(pool).unwrap_or_else(|_| { println!("try insert pool cell failure!") });
    // DB_POOL.set(mysql::Pool::new(db_url).expect(&format!("Error connecting to {}", &db_url)))
    //     .unwrap_or_else(|_| { info!("try insert pool cell failure!") });
    println!("初始化数据库线程池--------结束-------");

}

// 从链接链接池里面获取链接

pub fn get_connect() -> &'static Pool<Sqlite> {
    println!("从链接池获取数据库链接----------开始----------");
    let conn = DB_POOL.get().expect("Error get pool from OneCell<Pool>");
    println!("从链接池获取数据库链接----------结束----------");
    conn
}

#[derive(Serialize, Deserialize)]
pub struct Timer {
    time:String,
    id: i64,
}

