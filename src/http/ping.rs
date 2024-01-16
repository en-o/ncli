// https://crates.io/crates/pinger


use std::fmt::{Debug};
use tokio::net::UdpSocket;

///  wget 具体执行动作
/// # 参数
/// - ip : 检测ip
/// - port : 检测端口 [可选]
#[tokio::main]
pub(crate) async fn dispose_ping(ip: &String, port: &Option<u32>) {
    println!("检测地址 : ip: {ip:?}, 端口：{port:?}");
    match port {
        None => {
            // 指验证 ip是否正常
            match pinger::ping(ip.clone(), None){
                Ok(msg) => {
                    println!("aaa ====== {:?}",msg)
                }
                Err(_) => {
                    println!("请输入正确的IP")
                }
            }
        }
        Some(_) => {
            // 验证 ip 端口

            // 创建一个 UDP socket 并绑定到指定的 IP 地址和端口
            let socket = UdpSocket::bind((ip, port.unwrap())).await.unwrap();
            println!("Listening on {}:{:?}", ip, port);
            // 无限循环接收数据包
            loop {
                let mut buffer = vec![0u8; 1024];
                let (size, peer_address) = socket.recv_from(&mut buffer).await.unwrap();
                // 处理接收到的数据
                let data = &buffer[..size];
                println!("Received {} bytes from {}: {:?}", size, peer_address, data);
            }
        }
    }

}