use std::path::PathBuf;
use clap::Subcommand;
use crate::Cli;
use crate::http::scan::dispose_scan;
use crate::http::wget::dispose_wget;

/// http 相关的
/// cargo run -- scan --help / cargo run -- wget --help
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

        /// 目标端口 - 可选[空： 扫描所有端口] -p 3306
        #[arg(short, long)]
        port: Option<u16>,

        /// 指定扫描的端口范围- 可选[与port存在的情况下，此参数不生效] -s 3300 -s 3310
        /// 单个表示  [指定端口,MAX]
        /// 两个表示 [指定端口Min,指定端口Max]
        /// 多个则无效
        #[arg(short, long)]
        scope: Option<Vec<u16>>,
    },
}


/// 参数的执行程序
/// # 参数
/// - cli:Cli::parse()
pub(crate) fn run_it(cli: & Cli) {
    // 你可以检查是否存在子命令，如果找到就使用它们
    //  .\target\debug\ncli.exe Wget da
    match &cli.http {
        Some(HttpCommands::Wget { url, dir, rename }) => {
            dispose_wget(url, dir, rename)
        }
        Some(HttpCommands::Scan { ip, port, scope }) => {
            dispose_scan(ip, port, scope)
        }
        None => {
            println!("no use HttpCommands")
        }
    }
}


