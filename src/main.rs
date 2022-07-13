use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Command_type{
    a_command: bool,
    c_command: bool,
    l_command: bool,
    comment: bool,
}

fn main() {
    // 初期化
    let args: Vec<String> = env::args().collect();
    let mut order =  Command_type{
        a_command:false,
        c_command:false,
        l_command:false,
        comment:false
    };
    //　引数合わせる
    if args.len() != 2 {
        panic!("Not enough arguments");
    }
    // ファイルの読み込み
    let mut f = File::open(&args[1]).expect("Not open this file");
    let contents = read_file(&mut f);
    // 中身を一行ずつ取り出してA命令かC命令かを判断する
    for line in contents.lines() {
        // 先にコメントアウトがあるかを調べて、あればコメントより前にコマンド部分があるか調べる
        order.comment = is_comment(&line);
        //  空行とコメントをなくした文字列のみ取り出す
        let syntax: String = pick_str(&line, &order);
        is_order(&syntax, &order); 
        //  コメントアウトのみの行を飛ばす
        if order.comment {
            println!("found comment!!!!");
            continue;
        }

        if order.a_command {
            println!("{syntax} is A order!");
            // TODO: A命令のときの処理
        } else {
            println!("{syntax} is C order!");
            // TODO: C命令のときの処理
        }
    }

    println!("\n-----本文------\n{}", contents);
}

fn is_comment(line: &str) -> bool {
    // '/'が二個続いたらコメントと認識する
    let mut i: usize = 0;
    while i < line.len() {
        if line.chars().nth(i).unwrap() == '/' {
            if line.chars().nth(i + 1).unwrap() == '/' {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn pick_str(line: &str, order: &Command_type) -> String {
    // 　空行を含まない文字列のみを抽出する。
    let v: Vec<&str> = line.split_whitespace().collect();
    let mut result = String::new();
    // println!("It is bool ->{comment}");
    // とりあえず空を埋める
    for i in &v {
        result.push_str(i);
    }
    // コメントがある時
    if order.comment {
        let s: Vec<&str> = result.matches("//").collect();
        return s[0].to_string();
    }
    result
}

fn is_order(line: &str, order:  &mut Command_type) {
    // A命令C命令その他を判断するC:dest=comp;jmp '=' or ';'が含まれているかどうかで判断
    if line.chars().nth(0).unwrap() == '@'{
        order.a_command = true;
    }
}

fn read_file(f: &mut File) -> String {
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("failed to read the file");

    content
}
