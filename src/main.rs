use clap::Parser;
use crate::http::http::HttpCommands;

mod http;
mod demo;
mod system;

#[derive(Parser,Debug)]
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
    // http 相关 命令
    http::http::run_it(cli);

}
