//普通模式请加上"-a", 输出文件名称请加上"-v"
//支持racing日志记录以及命令行彩色输出
use regex::Regex;
use std::env;
use std::process;
use tracing::{info, error, warn};
use tracing_subscriber::FmtSubscriber;
extern crate colored;
use colored::*;


mod file_search;

fn main() {

    // 初始化 tracing 记录器和订阅器
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
    
    // 日志记录示例
    info!("This is an info message.");
    error!("This is an error message.");
    warn!("This is a warning message.");

    println!("This is a {} message", "normal".black());
    println!("This is a {} message", "red".red());
    println!("This is a {} message", "green".green());
    println!("This is a {} message", "yellow".yellow());

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("使用方式：{} <目标目录> <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    // if args.len() == 3 {
    //     args.push_str("-a");
    // }
    // &str类型只是借用，就算是&mut str，也只是对一块字符串的借用。没有管理空间的权利

    let pattern = &args[2];
    match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式'{}':{}", pattern, err);
            process::exit(1);
        }
    };

    match file_search::find_files(&args[1], pattern, &args[3]) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("未找到匹配项。");
            } else {
                println!("找到以下匹配项：");
                for file in matches {
                    println!("{}", file);
                }
            }
        }
        Err(error) => {
            eprintln!("发生错误：{}", error);
            process::exit(1);
        }
    }
}
