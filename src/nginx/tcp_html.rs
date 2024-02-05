use std::ops::Add;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir};


/// 静态文件服务器[代理html]
/// # 参数
/// - path : html根目录路径
/// - port : 访问端口
/// - prefix : 访问前缀
#[tokio::main]
pub(crate) async fn dispose_html(path: &Option<String>, port: &u16, prefix: &Option<String>) {

    // 处理html根目录路径
    let root_dir = "./".to_string();
    let path_str =  path.as_ref().unwrap_or(&root_dir);
    // 构建资源目录
    let serve_dir
        = ServeDir::new(path_str);
    // 构建前缀
    let def_prefix = "/".to_string();
    let  prefix_str = prefix.as_ref().unwrap_or(&def_prefix);
    println!("browse: http://127.0.0.1:{}{}", port, prefix_str);

    // 创建 tcp 连接
    let addr = format!("0.0.0.0:{}",port);
    let listener =  TcpListener::bind(addr).await.unwrap();
    // 接口
    let app = Router::new()
        // 将  `prefix_str`  URL 映射到  serve_dir 目录下的哪些文件
        .nest_service(prefix_str.as_str(), serve_dir.clone());
    // 启动服务
    axum::serve(listener, app).await.unwrap();

}