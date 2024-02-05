use axum::http::Response;
use axum::response::Html;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir};
use crate::nginx::tcp_proxy_handler::handle;


/// 静态文件服务器[代理html]
/// # 参数
/// - path : html根目录路径
/// - port : 访问端口
/// - prefix : 访问前缀
#[tokio::main]
pub(crate) async fn dispose_html(path: &String, port: &u16, prefix: &Option<String>, api_target: &Option<String>) {


    // 构建资源目录
    let serve_dir
        = ServeDir::new(path);
    // 构建前缀
    let mut prefix_str = "/";
    match prefix {
        Some(pr) => {
            prefix_str = pr;
        }
        _ => {}
    }
    println!("browse: http://127.0.0.1:{}{}", port, prefix_str);
    // 创建 tcp 连接
    // web
    let addr = format!("0.0.0.0:{}",port);
    let listener =  TcpListener::bind(addr).await.unwrap();

    let app = Router::new()
        // 将  `prefix_str`  URL 映射到  serve_dir 目录下的哪些文件
        .nest_service(prefix_str, serve_dir.clone());
    // 启动服务
    axum::serve(listener, app).await.unwrap();

}