use std::io::Read;

use directories::BaseDirs;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};



// 创建一个全局的DB_POOL，可以一直使用，启动的时候初始化即可
static DB_POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

// 初始化数据库链接池

pub async fn init_mysql_pool(db_url: &str) {
    println!("初始化数据库线程池--------开始-------");

    let mut path = String::from("");
    if let Some(base_dirs) = BaseDirs::new() {
        let appdata_dir = base_dirs.data_dir();
        path += appdata_dir.to_str().unwrap();
        println!("AppData directory: {:?}", appdata_dir);
    }
    path += "\\tk.tools";
    //let path = "C:\\Users\\tjw1t\\AppData\\Roaming\\tk.tools";

    if let Ok(metadata) = std::fs::metadata(path.clone() + "\\" + db_url) {
        if !metadata.is_file() {
            std::fs::create_dir_all(path.clone());
            std::fs::File::create(path.clone() + "\\" + db_url);
        }
    } else {
        std::fs::create_dir_all(path.clone());
        std::fs::File::create(path.clone() + "\\" + db_url);
    }


    let pool = SqlitePoolOptions::new().connect_lazy(&("sqlite:".to_string() + &path + "\\" + db_url)).ok().expect("链接失败");

    // 执行一个 SQL 查询，例如创建一个表
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS 'cron_title' (
        'id' TEXT NOT NULL,
        'name' TEXT,
        'content' TEXT,
        'cron_type' TEXT DEFAULT 'interval',
        'interval' INTEGER,
        'appointed_time' INTEGER,
        'is_use' INTEGER,
        'pid' TEXT NOT NULL DEFAULT '0',
        'category' TEXT NOT NULL,
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
      );
      
      CREATE TABLE IF NOT EXISTS 'grid_info' (
        'id' TEXT NOT NULL,
        'name' TEXT,
        'describe' TEXT,
        'uri' TEXT,
        'code' TEXT,
        'classify' TEXT,
        'is_sys' integer DEFAULT 0,
        'x' integer,
        'y' integer,
        'w' integer,
        'h' integer,
        'template_id' text,
        'run_code' TEXT,
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
        PRIMARY KEY ('id'),
        CONSTRAINT 'only' UNIQUE ('code' COLLATE BINARY ASC) ON CONFLICT FAIL
      );
      INSERT INTO 'main'.'cron_title' ('id', 'name', 'content', 'cron_type', 'interval', 'appointed_time', 'is_use', 'pid', 'category', 'create_time', 'creator_lid', 'creator_name', 'updater_lid', 'updater_name', 'up_ver', 'sort', 'tenant_id', 'deleted', 'update_time') VALUES ('-1', '常规', '常规', 'interval', 1, 0, 0, '-2', 'type', 1713774603, NULL, NULL, NULL, NULL, NULL, 0, NULL, 0, 1716454081);
      INSERT INTO 'main'.'cron_title' ('id', 'name', 'content', 'cron_type', 'interval', 'appointed_time', 'is_use', 'pid', 'category', 'create_time', 'creator_lid', 'creator_name', 'updater_lid', 'updater_name', 'up_ver', 'sort', 'tenant_id', 'deleted', 'update_time') VALUES ('0', '默认', '默认', 'interval', 1, 0, 0, '-1', 'type', 1713774603, NULL, NULL, NULL, NULL, NULL, 0, NULL, 0, 1716454328);

      ")
        .execute(&pool)
        .await;


    DB_POOL.set(pool).unwrap_or_else(|_| { println!("try insert pool cell failure!") });
    // DB_POOL.set(mysql::Pool::new(db_url).expect(&format!("Error connecting to {}", &db_url)))
    //     .unwrap_or_else(|_| { info!("try insert pool cell failure!") });
    println!("初始化数据库线程池--------结束-------");

}

// 从链接链接池里面获取链接

pub fn get_connect() -> &'static Pool<Sqlite> {
    //println!("从链接池获取数据库链接----------开始----------");
    let conn = DB_POOL.get().expect("Error get pool from OneCell<Pool>");
    //println!("从链接池获取数据库链接----------结束----------");
    conn
}

#[derive(Serialize, Deserialize)]
pub struct Timer {
    time:String,
    id: i64,
}

