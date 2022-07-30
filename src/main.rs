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
        // let syntax: String = pick_str(&line, &mut command);
        is_command(&line, &mut command); 
        //  コメントアウトのみの行を飛ばす
        if command.comment && !command.a && !command.c && !command.l {
            println!("found comment!!!!");
            continue;
        }

        let mut writer = String::new();

        if command.a {
            // println!("{line} is A command!");
            writer = get_a_value(&line);
            writer = convert_atobit(&writer);// string 戻り値
            command.a = false;
            // TODO: A命令のときの処理
        } else if command.c {
            println!("{line} is C command!");
            command.c = false;
            
            // TODO: C命令のときの処理
        }else{
            command.l = false;
            println!("{line} is synbol");
        }
        println!("{writer}");
    }

    println!("\n-----本文------\n{}", contents);
}

fn convert_atobit(writer: &String) -> String { // bool返すかも
    // 数値かシンボルか
    let mut symbol_flag = false;
    for (i, c) in writer.chars().enumerate(){
        if !c.is_digit(10) && i == 0{
            symbol_flag = true; 
            break;
        }else if i != 0 && !c.is_digit(10){
            panic!("syntax error: First letter is a number");
        }
    }
    let mut num: u32 = 0;
    if symbol_flag {
        // TODO: ラベルとか変数とかの処理 構文チェック
    }else {
        // 数字を数値に
        num = writer.parse().unwrap();
    }
    // let mut r_writer = String::new();
    // r_writer = 
    to_bit_str(&num)
}

fn to_bit_str(num: &u32) -> String{
    let mut cnt:i32 = 0;
    let mut tmp:u32 = 0;
    let mut u:u32 = num.clone();
    let mut r = String::new();
    while u > 0{
        tmp = &u % 2;
        r = tmp.to_string() + &r;
        cnt += 1;
        u = &u / 2;
    }
    let mut result = r.chars().rev().collect::<String>();

    while cnt < 16 {
        result.push_str("0");
        cnt += 1;
    }
    result.chars().rev().collect::<String>()
}


fn get_a_value(line:&str) -> String{
    let v: Vec<&str> = line.split_whitespace().collect();
    // 例外処理
    if v.len() != 1{
        if v[1].chars().nth(0).unwrap() != '/'{
            if v[1].chars().nth(1).unwrap() != '/'{
                panic!("syntax error :Contains whitespace\n{}",line);
            }
        }
    }
    // @以降の値をとりだして返す
    let mut result = String::new();
    for (i, c) in v[0].chars().enumerate(){
        if i == 0 && c == '@'{
            continue;
        }else if c == '@' {
            panic!("Syntax error");
        }
        result.push(c);
    }
    result

}

fn is_comment(line: &str) -> bool {
    // '/'が二個続いたらコメントと認識する
    // 余力があれば、for in に変える
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

fn is_command(line: &str, command:  &mut CommandType) {
    // A命令C命令その他を判断するC:dest=comp;jmp '=' or ';'が含まれているかどうかで判断
    let v: Vec<&str> = line.split_whitespace().collect();
    if v.len() == 1 || command.comment{
        if v[0].chars().nth(0).unwrap() == '@'{
            command.a = true;
            return();
        } else if v[0].chars().nth(0).unwrap() == '/'{
            return();
        }
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


// fn pick_str(line: &str, command: &mut CommandType) -> String {
//     // 　空行を含まない文字列のみを抽出する。
//     let v: Vec<&str> = line.split_whitespace().collect();
//     let mut result = String::new();
//     // とりあえず空を埋める
//     for i in &v {
//         result.push_str(i);
//     }
//     // コメントがある時
//     if command.comment {
//         // コメントより左にコマンドがあるかどうか判定し、
//         // コマンドならばコマンドのみを返す コマンドがないならリザルトを返す
//         // コマンドがあればコメントのフラグを消す
//         let s: Vec<&str> = result.splitn(2,         '/').collect();
//         println!("This comment is{}", s[0].to_string());
//         if s[0].len() == 0{
//             return result;
//         }
//         command.comment = false;
//         return s[0].to_string();
//     }
//     result
// }