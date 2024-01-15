use clap::Parser;
use crate::demo::Args;

mod demo;

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
