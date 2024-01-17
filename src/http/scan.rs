use std::net::{IpAddr, TcpStream};
use std::str::FromStr;

// https://zhuanlan.zhihu.com/p/630658880?utm_id=0
// 最大端口
const MAX: u16 = 65535;
// 最小端口
const MIN: u16 = 0;


///  扫描的 具体执行动作
/// # 参数
/// - ip : 目标ip
/// - port : 目标端口 可选[空： 扫描所有端口]
/// - scope : 指定扫描的端口范围- 可选[与port存在的情况下，此参数不生效]
#[tokio::main]
pub(crate) async fn dispose_scan(ip: &String, port: &Option<u16>, scope: &Option<Vec<u16>>) {
    // println!("目标地址 : ip: {ip:?}, 端口：{port:?}");
    let addr = IpAddr::from_str(ip).unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    println!("==================================================================================");
    match port {
        None => {
            match scope {
                None => {
                    // 扫描所有端口
                    verify_port_opens(addr);
                }
                Some(scopes) => {
                    // 扫描指定范围
                    let mut end_port = MAX;
                    let mut start_port = MIN;
                    if let Some(value) = scopes.get(0) {
                        // 索引存在，修改值
                        start_port = *value;
                    }
                    if let Some(value) = scopes.get(1) {
                        // 索引存在，修改值
                        end_port = *value;
                    }
                    verify_scope_port_open(addr, start_port, end_port);
                }
            }
        }
        Some(transport) => {
            // 扫描指定端口
            let addr = IpAddr::from_str(ip).unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
            verify_port_open(*transport, addr);
        }
    }
    println!("==================================================================================");
}


/// 验证TCP端口是否开放
/// # 参数
/// - port : 扫描端口
/// - addr : 扫描地址
fn verify_port_open(port: u16, addr: IpAddr) {
    match TcpStream::connect((addr, port)) {
        Ok(_) => {
            println!("\x1b[32;1mIP({}) || 端口({}) || 状态(open) \x1b[0m", addr.to_string(), port);
        }
        Err(error) => {
            println!("IP({}) || 端口({}) || 状态(close) || err({})", addr.to_string(), port, error.to_string());
        }
    }
}


/// 扫描端口
/// # 参数
/// - addr : 扫描地址
fn verify_port_opens(addr: IpAddr) {
    let mut port: u16 = MIN + 1;
    loop {
        verify_port_open(port, addr);
        // 退出条件
        if MAX == port {
            break;
        }
        port += 1;
    }
}


/// 扫描指定范围端口
/// # 参数
/// - addr : 扫描地址
/// - start_port : 开始端口
/// - end_port   : 结束端口
///
fn verify_scope_port_open(addr: IpAddr, mut start_port: u16, end_port: u16) {
    loop {
        verify_port_open(start_port, addr);
        // 退出条件
        if end_port == start_port {
            break;
        }
        start_port += 1;
    }
}

