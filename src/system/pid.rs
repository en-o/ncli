use std::str;
use netstat2::{AddressFamilyFlags, get_sockets_info, ProtocolFlags, ProtocolSocketInfo};
use sysinfo::{Pid, System};

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
                      process.cwd().map(|p| p.to_str()).unwrap_or_default(),
                );
            }
        }
        Some(assign_port) => {
            if let Some(pids) = pid_by_port(assign_port) {
                // 使用 for 循环输出 Vec 中的元素
                let mut system = System::new_all();
                // 刷新系统信息
                system.refresh_all();
                for pid in &pids {
                   let pid_info =   system.process(Pid::from_u32(*pid));
                    pid_info.map(|process| print(*pid,
                                 process.name(),
                                 &process.status().to_string(),
                                 process.cwd().map(|p| p.to_str()).unwrap_or_default(),
                    ));
                }
            } else {
                println!("当前端口未被占用")
            }
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


/// windows 专用的 端口换pid
/// https://crates.io/crates/netstat2
#[cfg(target_os = "windows")]
fn pid_by_port(port: &u16) -> Option<Vec<u32>> {
    let mut pids: Vec<u32> = Vec::new();
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();

    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                // println!(
                //     "TCP {}:{} -> {}:{} {:?} - {}",
                //     tcp_si.local_addr,
                //     tcp_si.local_port,
                //     tcp_si.remote_addr,
                //     tcp_si.remote_port,
                //     si.associated_pids,
                //     tcp_si.state
                // )
                if (tcp_si.local_port == *port) {
                    pids.extend(si.associated_pids.iter());
                }
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                // println!(
                //     "UDP {}:{} -> *:* {:?}",
                //     udp_si.local_addr, udp_si.local_port, si.associated_pids
                // )
                if (udp_si.local_port == *port) {
                    pids.extend(si.associated_pids.iter());
                }
            }
        }
        // 使用 vec.dedup() 进行就地去重
        pids.dedup();

    }
    Some(pids)
}
