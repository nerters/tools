
use std::fmt;
use std::fs::{self, File};
use std::path::Path;
use std::time::{Duration, Instant};

use image::imageops::FilterType;
use image::ImageFormat;
 
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

fn compress() {
    let path = "/temp/";
    let p_p = Path::new(path);
    let exts: Vec<&str> = vec!["png", "jpg", "jpeg"];
    let img_path = Path::new(path);
    let extension = match img_path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        _ => return,
    };
    if exts.contains(&extension) {//文件后缀判断
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
                return;
            }
        };
        let tiny = match image::open(img_path) {
            Ok(image) => image,
            _ => {
                println!(
                    "{} 压缩失败,图片格式有误，可以使用画图工具打开重新保存",
                    file_name
                );
                return;
            }
        };
        let scaled = tiny.resize(800, 600, FilterType::Triangle);//使用这个算法进行压缩
        let mut output = File::create(p_p.join(file_name).as_path()).unwrap();
        scaled.write_to(&mut output, ImageFormat::Png).unwrap();//都输出成jpg格式
    }
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
                if exts.contains(&extension) {//文件后缀判断
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
                    let scaled = tiny.resize(80, 600, FilterType::Triangle);//使用这个算法进行压缩
                    if let Some(name) = parts.first() {
                        let mut output = File::create(target_path.join(name.to_string() + ".WebP").as_path()).unwrap();
                        scaled.write_to(&mut output, ImageFormat::WebP).unwrap();//都输出成jpg格式
                    } else {
                        let mut output = File::create(target_path.join(file_name).as_path()).unwrap();   
                        scaled.write_to(&mut output, ImageFormat::Png).unwrap();//都输出成jpg格式
                    }
                }
            }
        }
    }
}