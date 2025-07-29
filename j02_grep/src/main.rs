use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{io, process};
use std::time::Instant;
use clap::{arg, Parser};
use colored::Colorize;

#[derive(Parser)]
#[command(version, about)]
struct GrepArgs{
    /*
    short           表示开启标记的短参数：-n
    long            默认长参数为 --number，这里配置为 --line_number
    default_value_t 设置默认值
     */
    /// 显示关键词所在行及其行号
    #[arg(short, long="line_number",default_value_t = false)]
    number: bool,

    /// 检索词
    keyword: String,
    /// 文件名
    path: String
}

fn main()-> io::Result<()> {
    let args = GrepArgs::parse();

    // 判断文件是否存在
    let path = Path::new(&args.path);
    if !path.metadata().map_or(false, |meta| meta.is_file()) {
        println!("⌈{}⌋ 不存在或不是一个文件😔", args.path.red());
        process::exit(1);
    }

    let started = Instant::now();
    let gray = (128, 128, 128);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(&args.keyword) {
            count += 1;

            // 格式化输出检索到的行（按需添加行号）
            println!("{}{}",
                if args.number { format!("{:<5} ", index+1).custom_color(gray).to_string()} else { String::new() },
                line.replace(&args.keyword, &args.keyword.magenta().to_string())
            )
        }
    }

    let output = format!("在 {} 找到{}个关键字，耗时{:.2?}", args.path, count, started.elapsed());
    println!("\n{}", output.custom_color(gray));

    Ok(())
}