use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let mut f = File::open(&args[1]).expect("Not open this file");
    let contents = read_file(&mut f);
    // TODO: 中身を一行ずつ取り出してA命令かC命令かを判断する
    for line in contents.lines() {
        // TODO: 空行をなくした文字列のみ取り出す
        // TODO: コメントアウトの行を飛ばす
        // if line.char
        if is_a_order(&line) {
            println!("{line} is A order!");
            // TODO: A命令のときの処理
        } else {
            println!("{line} is C order!");
            // TODO: C命令のときの処理
        }
    }

    println!("{}", contents);
}

fn is_a_order(line: &str) -> bool {
    //　空行を飛ばす処理を別関数に移すと思われる
    let mut first_string: usize = 0;
    let cs: String = line.to_string();
    for c in cs.as_str().chars(){
        if c == ' ' {
            first_string += 1;
        }
    }
    line.chars().nth(first_string).unwrap() == '@'
}

fn read_file(f: &mut File) -> String {
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("failed to read the file");

    content
}
