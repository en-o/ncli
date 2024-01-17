use clap::Parser;
use crate::command_enum::Commands;

mod demo;
mod command_enum;
mod system;
mod http;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}


fn main() {
    // 获取命令行参数
    let cli = Cli::parse();
    // 命令集合
    command_enum::run_it(cli);

}
