use clap::Parser;

/// Simple program to greet a person
/// 定义接受参数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: String,

    /// Number of times to greet
    /// 循环次数
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}


pub fn test(){
    // 获取接受参数
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}