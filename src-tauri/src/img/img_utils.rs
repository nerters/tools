use std::fs::{self, File};
use std::io::{BufWriter, Cursor, Error, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fmt, time};

use base64::Engine;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageEncoder, ImageFormat, RgbaImage};

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

pub fn resize_image(file_path: &str, nwidth: u32, nheight: u32) -> String {
    let exts: Vec<&str> = vec!["png", "jpg", "jpeg","PGN", "JPG", "JPEG", "webp", "WEBP"];
    let img_path = Path::new(file_path);

    let extension = match img_path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        _ => return "".to_string(),
    };
    if exts.contains(&extension) {
        //文件后缀判断
        let file_name = img_path.file_name().unwrap().to_str().unwrap();
        let path = &file_path.replace(file_name, "");
        let target_path = Path::new(&path);
        let target_path = &target_path.join("temp");
        let target_path = target_path.as_path();
        if !target_path.exists() {
            let _ = std::fs::create_dir_all(target_path);
        }
        let timer = Instant::now();
        println!("target by {} in {}", file_name, Elapsed::from(&timer));

        let tiny = match image::open(img_path) {
            Ok(image) => image,
            _ => {
                println!(
                    "{} 压缩失败,图片格式有误，可以使用画图工具打开重新保存",
                    file_name
                );
                return "".to_string();
            }
        };
        let scaled = tiny.resize(nwidth, nheight, FilterType::Triangle); //使用这个算法进行压缩
                                                                         //let mut output = File::create(target_path.join(file_name).as_path()).unwrap();
        let mut buffer: Vec<u8> = Vec::new();

        let parts: Vec<&str> = file_name.split('.').collect();
        if let Some(name) = parts.first() {
            // 创建一个 BufWriter 并将其与 Vec<u8> 关联
            let mut writer = BufWriter::new(Cursor::new(&mut buffer));
            match extension {
                "png" => {
                    scaled.write_to(&mut writer, ImageFormat::Png).unwrap(); //都输出成jpg格式
                }
                "webp" => {
                    scaled.write_to(&mut writer, ImageFormat::WebP).unwrap(); //都输出成jpg格式
                }
                _ => {
                    scaled.write_to(&mut writer, ImageFormat::Jpeg).unwrap(); //都输出成jpg格式
                }
            };
        } else {
            println!("获取文件名失败！");
            return "".to_string();
        }

        let b64_url = base64::prelude::BASE64_STANDARD.encode(buffer);

        return b64_url;
    }
    return "".to_string();
}



pub fn compress_image(file_path: &str, quality: u8) -> String {
        // 打开图片
    let img = image::open(file_path).unwrap();

    // 创建一个 buffer 用来存储压缩后的图片数据
    let mut buffer: Vec<u8> = Vec::new();

    // 创建一个 Cursor，并与 buffer 关联
    let cursor = Cursor::new(&mut buffer);

    // 创建一个 JpegEncoder，设置压缩质量为 10
    let mut encoder = JpegEncoder::new_with_quality(cursor, quality);


    // 将图片编码为 JPEG 格式并写入 buffer
    encoder
        .encode(
            img.as_bytes(),
            img.width(),
            img.height(),
            img.color().into(),
        )
        .unwrap();

    let b64_url = base64::prelude::BASE64_STANDARD.encode(buffer);

    return b64_url;
}


pub fn grayscale_image(file_path: &str) -> String {
    let img_path = Path::new(file_path);
    // 打开图片
    let img = image::open(img_path).unwrap();

    let img = img.grayscale();

    let extension = match img_path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        _ => return "".to_string(),
    };
    
    //文件后缀判断
    let file_name = img_path.file_name().unwrap().to_str().unwrap();
    let mut buffer: Vec<u8> = Vec::new();

    let parts: Vec<&str> = file_name.split('.').collect();
    if let Some(name) = parts.first() {
        // 创建一个 BufWriter 并将其与 Vec<u8> 关联
        let mut writer = BufWriter::new(Cursor::new(&mut buffer));
        match extension {
            "png" => {
                img.write_to(&mut writer, ImageFormat::Png).unwrap(); //都输出成jpg格式
            }
            "webp" => {
                img.write_to(&mut writer, ImageFormat::WebP).unwrap(); //都输出成jpg格式
            }
            _ => {
                img.write_to(&mut writer, ImageFormat::Jpeg).unwrap(); //都输出成jpg格式
            }
        };
    } else {
        println!("获取文件名失败！");
        return "".to_string();
    }

    let b64_url = base64::prelude::BASE64_STANDARD.encode(buffer);

    return b64_url;
}



#[test]
fn test() {
    let exts: Vec<&str> = vec!["png", "jpg", "jpeg"];
    use std::path::Path;
    //以当前目录为源目录，以当前目录+scale目录
    let src_path = Path::new("C:\\Img\\scale");
    let target_path = src_path.join("scale");
    let target_path = target_path.as_path();
    fs::create_dir_all(target_path).unwrap();

    for entry in src_path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let img_path = &path.as_path();
            println!("{}", img_path.to_string_lossy());
            if img_path.is_file() {
                println!("1");
                let extension = match img_path.extension() {
                    Some(ext) => ext.to_str().unwrap(),
                    _ => break,
                };
                if exts.contains(&extension) {
                    //文件后缀判断
                    println!("2");
                    let file_name = img_path.file_name().unwrap().to_str().unwrap();
                    let timer = Instant::now();
                    println!("target by {} in {}", file_name, Elapsed::from(&timer));

                    let tiny = match image::open(img_path) {
                        Ok(image) => image,
                        _ => {
                            println!(
                                "{} 压缩失败,图片格式有误，可以使用画图工具打开重新保存",
                                file_name
                            );
                            break;
                        }
                    };
                    let tiny = match image::open(img_path) {
                        Ok(image) => image,
                        _ => {
                            println!(
                                "{} 压缩失败,图片格式有误，可以使用画图工具打开重新保存",
                                file_name
                            );
                            break;
                        }
                    };
                    let parts: Vec<&str> = file_name.split('.').collect();
                    let scaled = tiny.resize(80, 600, FilterType::Triangle); //使用这个算法进行压缩
                    if let Some(name) = parts.first() {
                        let mut output =
                            File::create(target_path.join(name.to_string() + ".WebP").as_path())
                                .unwrap();
                        scaled.write_to(&mut output, ImageFormat::WebP).unwrap();
                    //都输出成jpg格式
                    } else {
                        let mut output =
                            File::create(target_path.join(file_name).as_path()).unwrap();
                        scaled.write_to(&mut output, ImageFormat::Png).unwrap();
                        //都输出成jpg格式
                    }
                }
            }
        }
    }
}


#[test]
fn test1() {
    // // 打开图片
    let img = image::open("C:\\Users\\tjw1t\\Pictures\\BB1lckIZ.jpg").unwrap();

    // 创建一个文件保存路径
    let output_path = "C:\\Users\\tjw1t\\Pictures\\111.jpg";

    // 创建文件，并使用 BufWriter 包装
    let output_file = File::create(output_path).unwrap();
    let writer = BufWriter::new(output_file);

    // 创建一个 JpegEncoder，并设置压缩质量为 10
    let mut encoder = JpegEncoder::new_with_quality(writer, 10);

    println!("width::::{}", img.width());
    println!("height::::{}", img.height());

    // 将图片编码为 JPEG 格式并写入 buffer
    encoder
    .encode(
        img.as_bytes(),
        img.width(),
        img.height(),
        img.color().into(),
    )
    .unwrap();
}






