use std::error::Error;

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// 处理请求转发
/// # 参数
/// - inbound : TcpStream
/// - proxy_addr : 代理地址 {192.168.1.71:9003}
pub async fn handle(mut inbound: TcpStream, proxy_addr: &str) -> Result<(), Box<dyn Error>> {
    println!("handle proxy: {}", proxy_addr);

    // 通过异步的方式连接到代理服务器
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    // 将输入和输出流分割为只读和只写部分,这样做是为了能够同时读取和写入数据，而不会发生冲突
    // 获取监听端的输入输出流
    let (mut ri, mut wi) = inbound.split();
    // 获代理端的输入输出流
    let (mut ro, mut wo) = outbound.split();


    // 将数据从一个读取器复制到一个写入器，直到到达读取器的末尾 [实现了从客户端到服务器和从服务器到客户端的数据传输]
    // 监听端的输入流复制到代理端的输出，完成参数传递 [从输入流复制数据到输出流]
    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };
    // 代理端的输入复制到监听端的输出，完成结果数据传递[从输出流复制数据到输入流]
    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };
    // 同时等待两个异步任务（即从客户端到服务器和从服务器到客户端的数据传输），并将任何错误返回给调用者
    tokio::try_join!(client_to_server, server_to_client)?;
    Ok(())
}