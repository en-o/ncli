// tcp 接口流量转发


/// 处理 pic 命令
/// # 参数
/// - port : 端口 [为空查所有]
pub(crate) fn dispose_ntp(local: &u16, url: &String ,port: &u16) {
    println!("本地端口{},目标地址{},目标端口{}", local, url, port)
}