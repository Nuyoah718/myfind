use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("使用方式：{}<目标目录><要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    let pattern = &args[2];
    let regex = match Regex::new(pattern){
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式'{}':{}", pattern, err);
            process::exit(1);
        }
    };

    match find(&args[1], &regex) {
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

fn find<P: AsRef<Path>>(root: P, regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches)?;
    Ok(matches)
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir(){
        for entry in fs::read_dir(dir)?{
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                walk_tree(&path, regex, matches)?;
            }else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}