use std::path::PathBuf;
use clap::Subcommand;
use crate::Cli;
use crate::http::scan::dispose_scan;
use crate::http::wget::dispose_wget;
use crate::nginx::tcp_html::dispose_html;
use crate::nginx::tcp_proxy::dispose_ntp;
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
        port: Option<u16>
    },

    /// nginx tcp proxy (接口代理)
    NTP {
        /// 本地端口 {e.g 8080}
        #[arg(short, long)]
        local: u16,

        /// 目标地址,默认http {e.g 192.168.1.1}
        #[arg(short, long)]
        url: String,

        /// 目标端口 {e.g 8080}
        #[arg(short, long)]
        port: u16,
    },

    /// nginx proxy page (页面代理)
    HTML {
        /// 访问端口 {e.g 8080}
        #[arg(short, long)]
        port: u16,

        /// html根目录路径   - 可选[默认当前执行目录 ./]
        #[arg(short, long)]
        assets: Option<String>,

        /// 访问前缀  - 可选[默认 /]
        #[arg(long)]
        prefix: Option<String>,

        /// 代理接口 - 可选 [为空不处理，e.g 192.168.1.1:8200 ,最后没有斜杠]
        #[arg(long)]
        proxy: Option<String>,

        /// 代理前缀 - 代理接口设置了这个前缀也必填 [127.0.0.1:port/proxyPrefix -> proxyApi]
        #[arg(long)]
        proxy_prefix: Option<String>,
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
        Some(Commands::Pid { port }) => {
            dispose_pid(port)
        }
        Some(Commands::NTP { local, url, port }) => {
            dispose_ntp(local, url, port)
        }
        Some(Commands::HTML { assets, port, prefix,proxy, proxy_prefix}) => {
            dispose_html(assets, port, prefix, proxy, proxy_prefix)
        }
        None => {
            println!("no use commands")
        }
    }
}