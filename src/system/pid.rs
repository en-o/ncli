use sysinfo::{Networks, System};
// https://crates.io/crates/sysinfo

/// 处理 pic 命令
/// # 参数
/// - port : 端口 [为空查所有]
pub(crate) fn dispose_pid(port: &Option<u16>) {
    match port {
        None => {
            let mut system = System::new_all();
            // 刷新系统信息
            system.refresh_all();
            for (pid, process) in system.processes() {
                print(pid.as_u32(),
                      process.name(),
                      &process.status().to_string(),
                      process.cwd().map(|p| p.to_str()).unwrap_or_default()
                );
            }
        }
        Some(assign) => {
            println!("assign: {assign:?}")
        }
    }
}


/// 统一打印
fn print(pid: u32, name: &str, status: &String, path: Option<&str>) {
    println!("IP(\x1b[32;1m{}\x1b[0m) || 应用({}) || 状态({})  || 路径({:?})",
             pid,
             name,
             status,
             path);
}

