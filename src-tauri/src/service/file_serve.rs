#[tauri::command]
pub fn read_file(path: String) -> String {
    println!("{}", path);
    match file::get_text(path) {
        Ok(text) => {
            return text;
        }
        Err(e) => {
            println!("获取数据失败！{}", e);
            return format!("#@#error#@#_{}", e);
        }
    }
}