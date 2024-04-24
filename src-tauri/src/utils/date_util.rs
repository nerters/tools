use std::fmt::Debug;

use chrono::{prelude::Utc, DateTime, Datelike};

pub fn get_now_time_ymd() -> String {
    //获取当前时间
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    year.to_string() + now.month().to_string().as_str() + now.day().to_string().as_str()
}

pub fn get_now_time_y_m_d() -> (u32, u32, u32) {
    //获取当前时间
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    (year, now.month(), now.day())
}



pub fn get_now_time_m() -> i64 {
    //获取当前时间
    let now = Utc::now();
    now.timestamp()
}


pub fn get_ymd_by_y_m_d(y :u32,m :u32, d :u32) -> String {
    y.to_string() + "年" + m.to_string().as_str() + "月" + d.to_string().as_str() + "日"
}



pub fn get_ymd_num_by_y_m_d(y :u32,m :u32, d :u32) -> i32 {
    let mut num = y.to_string();
    if m <10 {
       num.push_str("0");
    }
    num.push_str(m.to_string().as_str());
    if d < 10 {
        num.push_str("0");
    }
    num.push_str(d.to_string().as_str());
    let num:i32 = num.parse().unwrap();
    num
}



pub fn get_ymd_m_by_str(time: String) -> i64 {
    let date = DateTime::parse_from_str(&(time + " 00:00:01 +0000"), "%Y年%m月%d日 %H:%M:%S %z");
    match date {
        Ok(date) => {
            return date.timestamp();
        },
        Err(e) => {
            println!("{}", e);
            return 0;
        },
    }
}


pub fn get_ymd_str_by_m(time: i64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap();

    datetime.format("%Y-%m-%d").to_string()
}