

/// 处理 pic 命令
/// # 参数
/// - port : 端口 [为空查所有]
pub(crate) fn dispose_pid(port: &Option<u16>) {
    match port {
        None => {
            println!("pid_all")
        }
        Some(assign) => {
            println!("assign: {assign:?}")
        }
    }

    // let mut system = System::new_all();
    // system.refresh_all();
    //
    // for (pid, process) in system.get_processes() {
    //     println!("PID: {}, Name: {}, Status: {:?}", pid, process.name(), process.status());
    // }
}