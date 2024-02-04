use crate::nginx::tcp_proxy_connection::{Connection, Protocol};
use crate::nginx::tcp_proxy_listener::HttpListener;

/// tcp 接口流量转发
/// # 参数
/// - local : 本地端口
/// - url   : 目标地址
/// - port  : 目标端口
#[tokio::main]
pub(crate) async fn dispose_ntp(local: &u16, url: &String, port: &u16) {
    println!("本地端口{},目标地址{},目标端口{}", local, url, port);
    // 监听端口
    let l_conn = Connection::new("0.0.0.0", *local);
    // 转发接口
    let f_conn = Connection::new(url, *port);
    // 构建 HttpListener
    let tcp = HttpListener::new(l_conn, f_conn);
    // run
    tcp.listen().await.expect("接口转发失败");
}



