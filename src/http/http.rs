use std::path::PathBuf;
use clap::Subcommand;
use crate::Cli;
use crate::http::scan::dispose_scan;
use crate::http::wget::dispose_wget;

/// http 相关的
/// cargo run -- ping --help / cargo run -- wget --help
#[derive(Subcommand, Debug)]
pub enum HttpCommands {
    /// 下载文件
    Wget {
        /// 文件地址
        url: String,
        /// 存储位置 - 可选[空：当前执行路径下]
        #[arg(short, long)]
        dir: Option<PathBuf>,
        /// 自定义文件名称（不用写后缀）- 可选[空：文件原名]
        #[arg(short, long)]
        rename: Option<String>,
    },
    /// 扫描端口
    Scan {
        /// 目标IP
        ip: String,
        /// 目标端口 - 可选[空： 展示所有端口状态]
        #[arg(short, long)]
        port: Option<u16>,
    },
}


/// 参数的执行程序
/// # 参数
/// - cli:Cli::parse()
pub(crate) fn run_it(cli: Cli) {
    // 你可以检查是否存在子命令，如果找到就使用它们
    //  .\target\debug\ncli.exe Wget da
    match &cli.http {
        Some(HttpCommands::Wget { url, dir, rename }) => {
            dispose_wget(url, dir, rename)
        }
        Some(HttpCommands::Scan { ip, port }) => {
            dispose_scan(ip, port)
        }
        None => {
            println!("no use HttpCommands")
        }
    }
}


