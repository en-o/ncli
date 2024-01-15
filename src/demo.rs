use clap::Parser;

// clap ： https://www.cnblogs.com/SleepSupervisor/p/17757751.html


/// Simple program to greet a person
/// 字段就是命令行的参数名称
///
#[derive(Parser, Debug)]
// 制命令行展示的行为
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    /// arg 控制单个参数的信息
    #[arg(short, long)]
    pub name: String,

    /// Number of times to greet
    /// arg 控制单个参数的信息
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}


#[warn(dead_code)]
pub fn test(){
    // 获取接受参数
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}