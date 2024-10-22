use crate::img::img_utils::compress;

#[tauri::command]
pub fn compress_img(file_path: String, nwidth: u32, nheight: u32, img_type: String) -> String {
    compress(&file_path, nwidth, nheight, img_type)
}