use regex::Regex;
use std::{env,process};
use colored::*;
 
fn main() {
    let args: Vec<String> =env::args().collect();
    
    // 参数 1：搜索目录 参数 2：要搜索的正则表达式
    // 思考一下：如果用户输入的参数太多，应该怎么样？
    //将参数以每两个为一组，第一个参数为搜索目录，第二个参数为要搜索的正则表达式
    
    if args.len()<3||args.len()%2==0 {
        eprintln!("{} {} {}","使用方法：".red(),args[0].red()," <目标目录> <要搜索的正则表达式>".red());
        process::exit(1);
    }

    for count in 1..=args.len()/2 {
        let pattern = &args[2*count];
        let regex =match Regex::new(&pattern) {
            Ok(re) =>re,
            Err(err) => {
                eprintln!("{} {} {}", "无效的正则表达式".red(), pattern.red(), err);
                process::exit(1);
            }
        };
        match find_mod::find(&args[2*count-1],&regex) {
            Ok(matches) => {
                if matches.is_empty(){
                    println!("{}","未找到匹配项。".red());
                }else {
                    println!("{}","找到以下匹配项：".blue());
                    for file in matches {
                        println!("{}",file.yellow());
                    }
                }
            }
            Err(err) => {
                eprintln!("{} {}","发生错误：".red(),err);
                process::exit(1);
            }
        }
    }
}

pub mod find_mod {
use std::{fs,path::Path};
use regex::Regex;

pub fn find<P: AsRef<Path>>(root: P,regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches)?;
    Ok(matches)
    }

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>>{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            }else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(dir.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
    }
}