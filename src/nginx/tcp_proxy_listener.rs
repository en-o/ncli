use std::error::Error;
use tokio::net::TcpListener;
use crate::nginx::tcp_proxy_connection::Connection;
use crate::nginx::tcp_proxy_handler::handle;

// 监听client请求
pub struct HttpListener<'a> {
    listen_conn: Connection<'a>,
    forward_conn: Connection<'a>,
}


/// new 一个简单 tcpListener 监听请求
impl<'a> HttpListener<'a> {

    /// 构建 HttpListener
    /// listen_conn 监听端口
    /// forward_conn 转发接口
    pub fn new(listen_conn: Connection<'a>, forward_conn: Connection<'a>) -> HttpListener<'a> {
        HttpListener {
            listen_conn,
            forward_conn,
        }
    }

    /// 开始监听
    pub async fn listen(&self) -> Result<(), Box<dyn Error>> {
        let addr = format!("{}:{}", self.listen_conn.host, self.listen_conn.port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        while let Ok((inbound, _)) = listener.accept().await {
            let proxy_addr = format!("{}:{}", self.forward_conn.host, self.forward_conn.port);
            tokio::spawn(async move {
                match handle(inbound, &proxy_addr.clone()).await {
                    Ok(_) => println!("success"),
                    Err(e) => println!("error: {}", e),
                }
            });
        }

        Ok(())
    }
}
