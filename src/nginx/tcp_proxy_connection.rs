
pub enum Protocol {
    Default,
    Http,
    Http2,
}
pub enum Transport {
    Default,
    Tcp,
    Tls,
}
// 监听或者转发的链接
pub struct Connection<'a> {
    pub protocol: Protocol,   // 通信协议
    pub transport: Transport, // 传输协议
    pub host: &'a str,        // 主机（ip或者域名）
    pub port: u16,            // 端口
}


impl<'a> Connection<'a> {

    /// 构建 Connection
    /// # 参数
    /// - host : ip
    /// - port : 端口
    pub fn new(host: &str, port: u16) -> Connection {
        Connection {
            protocol: Protocol::Default,
            transport: Transport::Default,
            host,
            port,
        }
    }

    /// 构建 Connection
    /// # 参数
    /// - p : 通信协议
    /// - t : 传输协议
    /// - host : ip
    /// - port : 端口
    pub fn new2(p: Protocol, t: Transport, host: &str, port: u16) -> Connection {
        Connection {
            protocol: p,
            transport: t,
            host,
            port,
        }
    }
}