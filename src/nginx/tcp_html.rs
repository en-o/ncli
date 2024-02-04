use axum::response::Html;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};


/// 静态文件服务器[代理html]
/// # 参数
/// - path : html根目录路径
/// - port : 访问端口
/// - prefix : 访问前缀
#[tokio::main]
pub(crate) async fn dispose_html(path: &String, port: &u16, prefix: &Option<String>) {
    // 构建资源目录
    let serve_dir
        = ServeDir::new(path);

    let mut prefix_str = "/";
    // 构建前缀
    match prefix {
        Some(pr) => {
            prefix_str = pr;
        }
        _ => {}
    }
    let app = Router::new()
        .nest_service(prefix_str, serve_dir.clone());
    // 创建 tcp 连接
    let addr = format!("0.0.0.0:{}",port);

    println!("browse: http://127.0.0.1:{}{}", port, prefix_str);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
}