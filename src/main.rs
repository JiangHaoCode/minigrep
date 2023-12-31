use std::env;
use std::process;

use jiang_mini_grep::Config;

fn main() {
    // unwrap_or_else 错误处理
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("输入错误: {err}");
        process::exit(1); //退出代码终止当前进程
    });

    // let config = Config::build(env::args()).unwrap_or(|e| {
    //     println!("{e}");
    //     process::exit(1);
    // });

    // Err(str) 错误信息
    if let Err(e) = jiang_mini_grep::run(config) {
        println!("运行出错: {e}");
        process::exit(1); //退出代码终止当前进程
    }
}
