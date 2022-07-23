use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

struct CommandType{
    a: bool,
    c: bool,
    l: bool,
    comment: bool,
}

// fn init_flag(command: &mut CommandType){
//     command.a = false;
//     command.c = false;
//     command.l = false;
//     command.comment = false;
// }

fn main() {
    // 初期化
    let args: Vec<String> = env::args().collect();
    let mut command =  CommandType{
        a:false,
        c:false,
        l:false,
        comment:false
    };

    //　引数合わせる
    if args.len() != 2 {
        panic!("Not enough arguments");
    }
    // 書き出し用ファイル名を作る
    // let mut write_file_name = 
    // ファイルの読み込み
    let mut contents = String::new();
    match get_file_contents(&args[1]) {
        Ok(contents_rerurn) =>{
            println!("Ok: file_open");
            contents = contents_rerurn;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // 中身を一行ずつ取り出してA命令かC命令かを判断する
    for line in contents.lines() {
        // println!("{}",line.len());
        if line.len() == 0{
            continue;
        }
        // 先にコメントアウトがあるかを調べて、あればコメントより前にコマンド部分があるか調べる
        command.comment = is_comment(&line);
        // TODO: 改行のみの箇所を飛ばすようにする
    
        //  空行とコメントをなくした文字列のみ取り出す
        let syntax: String = pick_str(&line, &mut command);
        to_decide_command(&syntax, &mut command); 
        //  コメントアウトのみの行を飛ばす
        if command.comment {
            println!("found comment!!!!");
            continue;
        }
        if !is_command(&syntax) {
            panic!("Syntax error from {}", syntax);
        }

        if command.a {
            println!("{syntax} is A command!");
            command.a = false;
            // TODO: A命令のときの処理
        } else if command.c {
            println!("{syntax} is C command!");
            command.c = false;

            // TODO: C命令のときの処理
        }else{
            command.l = false;
            println!("{syntax} is synbol");
        }
    }

    println!("\n-----本文------\n{}", contents);
}

fn is_command(line: &String) -> bool{
    for c in line.chars(){
        if c.is_ascii_digit() || c.is_lowercase() || c.is_uppercase(){
            continue;
        } else if c.is_ascii() {
            if c == '@' || c == ';' || c == '.' || c == ':' || c == '$' || c == '_' || c == '(' || c == ')'{
                println!("aaaaaaaaaaa{}",c);
                continue;
            }
        } else{
            println!("ああああああああ{}",c);
            return false;
        }
    }
    true
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

fn pick_str(line: &str, command: &mut CommandType) -> String {
    // 　空行を含まない文字列のみを抽出する。
    let v: Vec<&str> = line.split_whitespace().collect();
    let mut result = String::new();
    // とりあえず空を埋める
    for i in &v {
        result.push_str(i);
    }
    // コメントがある時
    if command.comment {
        // コメントより左にコマンドがあるかどうか判定し、
        // コマンドならばコマンドのみを返す コマンドがないならリザルトを返す
        // コマンドがあればコメントのフラグを消す
        let s: Vec<&str> = result.splitn(2, '/').collect();
        println!("This comment is {}", s[1].to_string());
        if s[0].len() == 0{
            return result;
        }
        command.comment = false;
        return s[0].to_string();
    }
    result
}

fn to_decide_command(line: &str, command:  &mut CommandType) {
    // A命令C命令その他を判断するC:dest=comp;jmp '=' or ';'が含まれているかどうかで判断
        if line.chars().nth(0).unwrap() == '@'{
            command.a = true;
            return();
        }
        for c in line.chars() {
            if c == '=' || c == ';' {
                command.c = true;
                return();
            }
        }
        command.l = true;
}

fn get_file_contents(file_path: &String) -> Result<String, Error>{
    let mut f = File::open(file_path.trim())?;
    let mut return_contents = String::new();
    f.read_to_string(&mut return_contents)?;
    Ok(return_contents)
}
