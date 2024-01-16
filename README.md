# ncli
rust 构建的命令行工具

# 使用说明
https://www.yuque.com/g/tanning/fquuhx/wr032rqrkdf13uci/collaborator/join?token=dShKyyTkcc2Fo3tT&source=doc_collaborator#

# 依赖
## [命令行库 clap](https://docs.rs/clap/latest/clap/)
> 使用 #[] 语法定义的属性注解  

### [属性分类](https://docs.rs/clap/4.2.1/clap/_derive/index.html#terminology)
#### Raw attributes
> 
- derive
- command [建一个命令行界面]
- arg [命令行参数的抽象表示]


#### Magic attributes
>  控制每一个具体的功能项目
- author
- version
- long
- short

# demo 
> windows 
1. `cargo build`
2. `.\target\debug\ncli.exe --help` 或者 `cargo run --  --help` 
```shell
PS G:\Rust\ncli> .\target\debug\ncli.exe --help
Simple program to greet a person

Usage: ncli.exe [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version

```
```shell
PS G:\Rust\ncli> .\target\debug\ncli.exe -n tan -c 2
Hello tan!
Hello tan!

```


