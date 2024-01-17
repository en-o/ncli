use std::env;
use std::fs::{File};
use std::io::{Write};
use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use url::Url;

///  wget 具体执行动作
/// # 参数
/// - url : 文件地址
/// - dir : 存储路径 [可选]
/// - rename : 自定义文件名
#[tokio::main]
pub(crate) async fn dispose_wget(url: &String, dir: &Option<PathBuf>, rename: &Option<String>) {
    let mut dir_path = PathBuf::new();
    // 处理存储路径
    match dir {
        Some(input_dir) => {
            dir_path.push(input_dir);
        }
        None => {
            // 获取当前工作目录的路径
            if let Ok(current_dir) = env::current_dir() {
                dir_path.push(current_dir);
            } else {
                // 当前路径获取失败
                println!("Failed to get current working directory.");
            }
        }
    }

    let file_name = find_file_name(url, rename);
    let save_path = format!("{}/{}", dir_path.to_string_lossy().to_string(), file_name);
    // println!("当前下载文件url: {url:?}, 存储路径：{save_path:?}");
    download(url, save_path, file_name).await;
}


/// 拿文件名+后缀
fn find_file_name(url: &String, rename: &Option<String>) -> String {
    let mut file_name = String::new();

    // 处理自定义名字
    match rename {
        None => {
            // 使用原来的名字
            // 获取url上的文件名
            if let Ok(parsed_url) = Url::parse(url) {
                // 获取 URL 路径的片段
                if let Some(last_segment) = parsed_url.path_segments().and_then(|segments| segments.last()) {
                    file_name = last_segment.to_string();
                }
            }
        }
        Some(rn) => {
            // 获取文件名的后缀
            let mut suffix = "";
            if let Some(extension) = Path::new(url).extension() {
                if let Some(extension_str) = extension.to_str() {
                    suffix = extension_str;
                }
            }
            file_name = rn.to_string() + "." + suffix;
        }
    }
    // 返回
    file_name
}


/// 下载文件
/// - url : 文件地址(http://127.0.0.1/xx.json)
/// - dir : 存储路径（c://x/tan//xx.json）
/// - file_name： 文件名（xx.json）
async fn download(url: &String, dir: String, file_name: String) {

    // 发送 GET 请求并获取响应
    let mut response = Client::new().get(url).send().await.expect("文件地址连接失败");
    // 检查响应状态
    if response.status().is_success() {

        // 获取响应体的大小
        let total_size = response.content_length().unwrap_or(0);
        // 创建进度条 https://baijiahao.baidu.com/s?id=1620454960836172917
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner} {msg}: [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").expect("进度条加载失败")
            .progress_chars("#>-"));
        pb.set_message(format!("Downloading ===> {}", file_name));
        let mut out_file = File::create(dir).expect("文件创建失败");

        // 读取响应并更新进度条
        let mut downloaded = 0;
        while let Some(chunk)  = response.chunk().await.unwrap(){
            downloaded += chunk.len();
            pb.set_position(downloaded as u64);
            let _ = out_file.write_all(&*chunk);
        }
         // 打开文件并将响应体保存到文件
        // copy(&mut response.bytes().await.unwrap().as_ref(), &mut out_file).expect("下载文件失败");
        // 完成后关闭进度条
        pb.finish_with_message(format!("Download complete ===> {}", file_name));
    } else {
        println!("[{}]下载失败: {}", url, response.status().to_string())
    }

}