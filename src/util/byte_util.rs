
/// 字节转换成mb
/// # 参数
/// - bytes: 字节
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0
}