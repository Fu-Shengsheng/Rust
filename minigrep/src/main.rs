// std::env::args 库提供了读取命令行参数的函数
use std::env;
use std::process;

use minigrep::Config;

/**
    main 函数中的责任应该被限制为：
    使用参数值调用命令行解析逻辑
    设置任何其他的配置
    调用 lib.rs 中的 run 函数
    如果 run 返回错误，则处理这个错误
*/

fn main() {
    // 将命令行参数收集到vector中
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 打印到标准错误流
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // 使用 if let 来检查run是否返回一个Err值
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:{}", e);

        process::exit(1);
    }
}
