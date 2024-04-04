use std::sync::Arc;
use axum::http::{StatusCode, Uri};
use axum::{
    response::{IntoResponse},
    Router,
};
use axum::response::{Redirect};
use axum::routing::any;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir};


struct RedirectParams {
    third_party_api: String,
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
            let state = Arc::new(RedirectParams { third_party_api:third_party_api.to_string() });
            match proxy_prefix {
                Some(pp) => {
                    let proxy_prefix_all = format!("/{}/*path", pp);
                    app = app
                        .route(
                            &*proxy_prefix_all,
                            // any(forward_api)
                            // any(redirect_api)
                            any(|uri: Uri| redirect_api(uri, state))
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

// /// 转发接口
// async fn forward_api(uri: Uri, third_party_api: &str, req: Request<BoxBody>)  -> Result<Response, Infallible> {
//     let proxy_prefix_all = format!("http://{}/","192.168.0.15:8200");
//     // 构造新的 URI
//     let new_uri = proxy_prefix_all.to_owned() + &uri.path().trim_start_matches('/');
//     println!("forward_api: {}", new_uri);
//     // 重定向到第三方接口
//     let client = Client::new();
//     let mut forwarded_req = Request::new(req.into_body());
//     *forwarded_req.uri_mut() = new_uri.parse()?;
//
//     let forwarded_resp = client
//         .request(forwarded_req.method().clone(), forwarded_req.uri().clone())
//         .body(forwarded_req.into_body())
//         .await
//         .map_err(|_| Infallible)?;
//
//     let (parts, body) = forwarded_resp.into_parts();
//     let status = parts.status;
//     let headers = parts.headers;
//
//     let response = Response::from_parts(status, headers, body.boxed());
//
//     Ok(response)
// }


/// 全局404页面
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "接口不存在")
}