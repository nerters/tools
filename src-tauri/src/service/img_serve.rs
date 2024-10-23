use crate::img::img_utils::{compress_image, grayscale_image, resize_image};

#[tauri::command]
pub fn compress_img(file_path: String, nwidth: u32, nheight: u32, type_f: String, quality: u8) -> String {
    println!("{}", quality);
    if type_f.eq_ignore_ascii_case("size") {
        return resize_image(&file_path, nwidth, nheight);
    } else if type_f.eq_ignore_ascii_case("compress") {
        return compress_image(&file_path, quality);
    }
    else if type_f.eq_ignore_ascii_case("grayscale") {
        return grayscale_image(&file_path);
    }
    resize_image(&file_path, nwidth, nheight)
}