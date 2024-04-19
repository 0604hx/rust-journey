use ansi_term::Color::{Blue, Purple, Red};

pub fn info(msg:&str){
    println!("{} {}", Blue.paint("INFO"), msg)
}

pub fn debug(msg:&str) {
    println!("{} {}", Purple.paint("DEBUG"), msg)
}

pub fn error(msg:&str){
    println!("{} {}", Red.paint("ERROR"), msg)
}