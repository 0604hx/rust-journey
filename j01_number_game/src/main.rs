use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("欢迎来到 Rust 版猜数字游戏");
    //创建随机数
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut limit = 8;

    println!("游戏开始（可尝试 {limit} 次），请输入你猜测的数字：");
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取用户输入失败");

        let guess:u32 = match  guess.trim().parse() {
            Ok(num)=> num,
            Err(e)=> {
                println!("数值转换出错 {:?}", e);
                continue
            }
        };

        match secret.cmp(&guess).reverse() {
            Ordering::Less => { println!("你猜的是 {guess} ，小了点噢，再猜下：") }
            Ordering::Greater => { println!("你猜的是 {guess} ，大了点噢，再猜下：") }
            Ordering::Equal => {
                println!("恭喜，猜对了 O(∩_∩)O");
                break
            }
        }

        limit -= 1;
        if limit <= 0 {
            println!("没有可用次数，游戏结束，你没有获得胜利 =.=");
            break
        }
    }
}
