use serde_json::{Map, Value};

pub fn json_diffv2(old: &Value, new: &Value) -> Value {
    match (old, new) {
        (&Value::Object(ref old), &Value::Object(ref new)) => {
            let mut changes = Map::new();

            for (key, new_value) in new {
                match old.get(key) {
                    Some(old_value) => {
                        if new_value != old_value {
                            changes.insert(key.clone(), json_diffv2(old_value, new_value));
                        }
                    }
                    None => {
                        changes.insert(key.clone(), new_value.clone());
                    }
                }
            }

            for (key, old_value) in old {
                if new.get(key).is_none() {
                    changes.insert(key.clone(), Value::Null);
                }
            }

            Value::Object(changes)
        }
        (&Value::Array(ref old), &Value::Array(ref new)) => {
            let mut changes = Vec::new();

            for (i, new_value) in new.iter().enumerate() {
                match old.get(i) {
                    Some(old_value) => {
                        if new_value != old_value {
                            changes.push(json_diffv2(old_value, new_value));
                        }
                    }
                    None => {
                        changes.push(new_value.clone());
                    }
                }
            }

            Value::Array(changes)
        }
        (old, new) => {
            if old != new {
                new.clone()
            } else {
                Value::Null
            }
        }
    }
}

pub fn json_diff(old: &Value, new: &Value) -> (Value, Value) {
    match (old, new) {
        (&Value::Object(ref old), &Value::Object(ref new)) => {
            let mut rest_old = Map::with_capacity(old.len());
            let mut old_temp = Map::with_capacity(old.len());
            let mut rest_new = Map::with_capacity(new.len());
            for (key, new_value) in new {
                match old.get(key) {
                    Some(old_value) => {
                        println!("new中key:{}", key);
                        if !new_value.eq(old_value) {
                            println!("new中key:{},值与old不相同", key);
                            let (old_data, new_data) = json_diff(old_value, new_value);
                            rest_new.insert(key.to_string(), new_data);
                            old_temp.insert(key.to_string(), old_data);
                        } else {
                            println!("new中key:{},值与old相同", key);
                            rest_new.insert(key.to_string(), new_value.clone());
                            old_temp.insert(key.to_string(), old_value.clone());
                        }
                    }
                    None => {
                        rest_new.insert(
                            "<span style='color: red'>".to_string() + key,
                            format!("{}</span>", new_value).into(),
                        );
                        //changes.insert(key.clone(), new_value.clone());
                    }
                }
            }

            for (key, old_value) in old {
                if new.get(key).is_none() {
                    println!("old中key:{},在new中没有", key);
                    rest_old.insert(
                        "<span style='color: red'>".to_string() + key,
                        format!("{}</span>", old_value).into(),
                    );
                    //old.insert(key.clone(), Value::Null);
                } else {
                    println!("old中key:{},在new中有", key);
                    match old_temp.get(key) {
                        Some(old_value) => {
                            println!("old中key:{},在old_temp中有", key);
                            rest_old.insert(key.to_string(), old_value.clone());
                        }
                        None => {
                            println!("old中key:{},在old_temp中没有", key);
                            rest_old.insert(
                                "<span style='color: red'>".to_string() + key,
                                format!("{}</span>", old_value).into(),
                            );
                        }
                    }
                }
            }

            (Value::Object(rest_old), Value::Object(rest_new))
        }
        (&Value::Array(ref old), &Value::Array(ref new)) => {
            let mut rest_old: Vec<Value> = vec![];
            let mut rest_new: Vec<Value> = vec![];
            let mut temp_max = new;
            let mut temp_min = old;
            if old.len() > new.len() {
                temp_max = old;
                temp_min = new;
            }

            for (i, new_value) in temp_max.iter().enumerate() {
                match temp_min.get(i) {
                    Some(old_value) => {
                        println!("array中找到对应位置的值");
                        if !new_value.eq(old_value) {
                            let (old_data, new_data) = json_diff(old_value, new_value);
                            rest_new.push(new_data);
                            rest_old.push(old_data);
                        } else {
                            rest_new.push(new_value.clone());
                            rest_old.push(old_value.clone());
                        }
                    }
                    None => {
                        rest_new
                            .push(format!("<span style='color: red'>{}</span>", new_value).into());
                    }
                }
            }

            (
                Value::Array(rest_old.clone()),
                Value::Array(rest_new.clone()),
            )
        }
        (old, new) => {
            println!("old::{}", old);
            println!("new::{}", new);
            if !old.eq(new) {
                println!("不相等");
                (
                    format!(
                        "<span style='color: red'>{}</span>",
                        old.to_string().replace("\"", "").replace("\'", "")
                    )
                    .into(),
                    format!(
                        "<span style='color: red'>{}</span>",
                        new.to_string().replace("\"", "").replace("\'", "")
                    )
                    .into(),
                )
            } else {
                println!("相等");
                (old.clone(), new.clone())
            }
        }
    }
}

#[test]
fn main1() {
    let old_json = r#"{"name": "John", "age": 30, "data":{"file_stuts":"0","file_uri":"/KHWJ/202404/1617505367192.jpeg","file_md5_code":"AF6D6C0EF97DE566DE171617B0CB7B0E","create_time":"2024-04-04 01:17:23","cr_user_name":"admin","suffix":"jpeg","prn":"1617505367192","file_size":52,"loginUserName":"admin","absolute_path":"","resource_ngx_uri":1617505367192,"file_type":"jpeg","parent_id":1615654004312,"name":"1712164641698.jpeg","id":1617505367192}}"#;
    let new_json = r#"{"name": "John", "age": 30, "data":{"file_stuts":"0","file_uri":"/KHWJ/202404/161750536719211.jpeg","file_md5_code":"AF6D6C0EF97DE566DE171617B0CB7B0E","create_time":"2024-04-04 01:17:23","cr_user_name":"admin","suffix":"jpeg","prn":"1617505367192","file_size":52,"loginUserName":"admin","absolute_path":"","resource_ngx_uri":1617505367192,"file_type":"jpeg","parent_id":1615654004312,"name":"1712164641698.jpeg","id":1617505367192}}"#;

    let old: Value = serde_json::from_str(old_json).unwrap();
    let new: Value = serde_json::from_str(new_json).unwrap();

    let (o, n) = json_diff(&old, &new);
    println!("{:?}", o);
    println!("{:?}", n);
}
