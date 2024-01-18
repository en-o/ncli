use std::path::PathBuf;
use clap::Subcommand;
use crate::Cli;
use crate::http::scan::dispose_scan;
use crate::http::wget::dispose_wget;
use crate::system::pid::dispose_pid;


///  系统 相关的
/// cargo run -- scan --help / cargo run -- wget --help
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 扫描
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

    /// 下载
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

    /// pid
    Pid {
        /// 查看指定端口的pid 可选[为空查询所有]
        #[arg(short, long)]
        port:Option<u16>
    },


}



/// 参数的执行程序
/// # 参数
/// - cli:Cli::parse()
pub(crate) fn run_it(cli: Cli) {
    // 你可以检查是否存在子命令，如果找到就使用它们
    // cargo run -- command  --help
    match &cli.command {
        Some(Commands::Wget { url, dir, rename }) => {
            dispose_wget(url, dir, rename)
        }
        Some(Commands::Scan { ip, port, scope }) => {
            dispose_scan(ip, port, scope)
        }
        Some(Commands::Pid { port}) => {
            dispose_pid(port)
        }
        None => {
            println!("no use commands")
        }
    }

}