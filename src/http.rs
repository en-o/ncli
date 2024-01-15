use clap::Subcommand;

/// http 相关的
#[derive(Subcommand)]
pub enum HttpCommands {
    /// 下载文件
    wget { url: Option<String> },
}