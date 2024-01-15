use clap::Parser;
use crate::http::HttpCommands;

mod http;
mod demo;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /*
       http网络相关 [此参数可选]
    */
    #[command(subcommand)]
    http: Option<HttpCommands>,
}


fn main() {
    // 获取命令行参数
    let cli = Cli::parse();

    // 你可以检查是否存在子命令，如果找到就使用它们
    match &cli.http {
        Some(HttpCommands::wget { download }) => {
            println!("下载地址是 : {download:?}")
        },
        None => {
            println!("no use")
        }
    }
}
