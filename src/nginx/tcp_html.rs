use std::error::Error;
use std::sync::Arc;
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode, Uri};
use axum::{
    response::{IntoResponse},
    Router,
};
use axum::body::Body;
use axum::extract::Request;
use axum::response::{Redirect, Response};
use axum::routing::any;
use reqwest::Client;
use tokio::net::TcpListener;
use tower::Service;
use tower_http::services::{ServeDir};


struct RedirectParams {
    third_party_api: String,
    client: Client
}

/// 静态文件服务器[代理html]
/// # 参数
/// - path : html根目录路径
/// - port : 访问端口
/// - prefix : 访问前缀
/// - proxy : 代理接口 - 可选 [为空不处理，e.g 192.168.1.1:8200]
/// - proxy_prefix : 代理前缀 - 可选 [127.0.0.1:port/proxyPrefix -> proxyApi]
#[tokio::main]
pub(crate) async fn dispose_html<'a>(path: &Option<String>
                                     , port: &u16
                                     , prefix: &Option<String>
                                     , proxy:  &Option<String>
                                     , proxy_prefix: &Option<String>) {

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

    let mut app =  Router::new();

    match proxy{
        Some(third_party_api) => {
            let client = Client::new();
            let state = Arc::new(RedirectParams { third_party_api:third_party_api.to_string(), client });
            match proxy_prefix {
                Some(pp) => {
                    let proxy_prefix_all = format!("/{}/*path", pp);
                    app = app
                        .route(
                            &*proxy_prefix_all,
                            any(|uri: Uri| forward_api(uri, state))
                            // any(|uri: Uri| redirect_api(uri, state))
                        )
                        .nest_service(prefix_str, serve_dir.clone())
                        .fallback(handler_404);
                }
                None => {
                    // 接口 [  将  `prefix_str`  URL 映射到  serve_dir 目录下的哪些文件]
                    app = Router::new()
                        .nest_service(prefix_str.as_str(), serve_dir.clone())
                        // 访问不存在的接口 统一返回404
                        .fallback(handler_404);
                }
            }
        }
        None => {
            // 接口 [  将  `prefix_str`  URL 映射到  serve_dir 目录下的哪些文件]
            app = Router::new()
                .nest_service(prefix_str.as_str(), serve_dir.clone())
                // 访问不存在的接口 统一返回404
                .fallback(handler_404);
        }
    }

    // 启动服务
    axum::serve(listener, app).await.unwrap();

}

/// 重定向
/// # 参数
/// - uri : Uri
/// - third_party_api : 192.168.0.15:8200
async fn redirect_api<'a>(uri: Uri, state: Arc<RedirectParams>) -> Redirect {
    let proxy_prefix_all = format!("http://{}/",state.third_party_api);
    // 构造新的 URI
    let new_uri = proxy_prefix_all.to_owned() + &uri.path().trim_start_matches('/');
    println!("forward_api: {}", new_uri);
    // 重定向到第三方接口
    Redirect::temporary(&new_uri)
}

// 定义一个处理函数，用于转发请求
async fn forward_api<'a>(uri: Uri, state: Arc<RedirectParams>) -> Response  {
    let proxy_prefix_all = format!("http://{}/",state.third_party_api);
    // 构造新的 URI
    let new_uri = proxy_prefix_all.to_owned() + &uri.path().trim_start_matches('/');
    println!("forward_api: {}", new_uri);

    let reqwest_response = match state.client.get(new_uri).send().await {
        Ok(res) => res,
        Err(err) => {
            tracing::error!(%err, "request failed");
            return (StatusCode::BAD_REQUEST, Body::empty()).into_response();
        }
    };
    let response_builder = Response::builder().status(reqwest_response.status().as_u16());

    // Here the mapping of headers is required due to reqwest and axum differ on the http crate versions
    let mut headers = HeaderMap::with_capacity(reqwest_response.headers().len());
    headers.extend(reqwest_response.headers().into_iter().map(|(name, value)| {
        let name = HeaderName::from_bytes(name.as_ref()).unwrap();
        let value = HeaderValue::from_bytes(value.as_ref()).unwrap();
        (name, value)
    }));

    response_builder
        .body(Body::from_stream(reqwest_response.bytes_stream()))
        // This unwrap is fine because the body is empty here
        .unwrap()
}



/// 全局404页面
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "接口不存在")
}