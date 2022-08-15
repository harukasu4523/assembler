use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::process;
use std::collections::HashMap;

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
    
    // ファイルの読み込み
    let mut contents = String::new();
    match get_file_contents(&args[1]) {
        Ok(contents_rerurn) =>{
            println!("Ok: file_open");
            contents = contents_rerurn;
        }
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }

    // TODO: シンボルテーブルの作成とプログラムカウンタの作成
    let mut map:HashMap<&str, i32> =HashMap::new(); 

    // 定義済みシンボルをシンボルテーブルに格納する
    init_table(&mut map);

    let mut pc:u32 = 0;
    for line in contents.lines(){
        if line.len() == 0{
            continue;
        }
        // 先にコメントアウトがあるかを調べて、あればコメントより前にコマンド部分があるか調べる
        command.comment = is_comment(&line);
    
        is_command(&line, &mut command); 
        //  コメントアウトのみの行を飛ばす
        if command.c || (command.comment && !command.a && !command.l) {
            println!("found comment!!!!");
            continue;
        }
        let mut symbol = String::new();
        // TODO: ラベルが来たら、pc + 1の値をマップの要素として格納する
        if command.l {
            symbol = get_label(&line);
            command.l = false;
        }else{
            symbol = get_a_value(&line);
            pc += 1;
            command.a = false;
        }
        println!("{symbol}");
    }

    // 書き出し用ファイルを作る
    let mut w_file;
    match create_w_file(&args[1]){
        Ok(return_file) =>{
            println!("Ok: file create");
            w_file = return_file;
        }
        Err(e) => {
            println!("Error: {}",e);
            process::exit(1);
        }
    }
    
    // 中身を一行ずつ取り出してA命令かC命令かを判断する
    for line in contents.lines(){
        // println!("{}",line.len());
        if line.len() == 0{
            continue;
        }
        // 先にコメントアウトがあるかを調べて、あればコメントより前にコマンド部分があるか調べる
        command.comment = is_comment(&line);
    
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
        } else if command.c {
            // println!("{line} is C command!");
            writer = check_c_parse(&line);
            command.c = false;
            
        }else{
            command.l = false;
            println!("{line} is synbol");
            continue;
        }
        write!(w_file,"{writer}\n").expect("error");
    }

    println!("\n-----本文------\n{}", contents);
}


fn get_label(line: &str) -> String{
    let v: Vec<&str> = line.split_whitespace().collect();
    // 例外処理
    if v.len() != 1{
        if v[1].chars().nth(0).unwrap() != '/'{
            if v[1].chars().nth(1).unwrap() != '/'{
                println!("syntax error :Contains whitespace\n{line}");
                process::exit(1);
            }
        }
    }
    let mut s = String::new();
    for (i, c) in line.chars().enumerate(){
        if i == 0{
            continue;
        }
        if c == ')'{
            break;
        }
        s.push(c);
    }
    s
}

// ここの仮引数は加工済みのものを送ること。
fn check_symbol(writer: &String) -> bool { // bool返す
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
        parse_check(writer);
        return true;
    }else {
        // 数字を数値に
        return false;
    }
    // let mut r_writer = String::new();
    // r_writer = 
}


fn init_table(map: &mut HashMap<&str, i32>){
    map.insert("SP", 0);
    map.insert("LCL", 1);
    map.insert("ARG", 2);
    map.insert("THIS", 3);
    map.insert("THAT", 4);
    map.insert("R0", 0);
    map.insert("R1", 1);
    map.insert("R2", 2);
    map.insert("R3", 3);
    map.insert("R4", 4);
    map.insert("R5", 5);
    map.insert("R6", 6);
    map.insert("R7", 7);
    map.insert("R8", 8);
    map.insert("R9", 9);
    map.insert("R10", 10);
    map.insert("R11", 11);
    map.insert("R12", 12);
    map.insert("R13", 13);
    map.insert("R14", 14);
    map.insert("R15", 15);
    map.insert("SCREEN", 16384);
    map.insert("KBD", 245756);
}

fn check_c_parse(s: &str)-> String{
    let mut position:u32 = 0;
    let mut f_cmp = false;
    let mut comp = String::new();
    let mut dst = String::new();
    let mut jmp = String::new();

    // TODO: cmp
    for c in s.chars(){
        if c.is_whitespace(){
            continue;
        }
        if c.is_lowercase(){
            println!("Error: parse error");
            process::exit(1);
        }
        if (c == 'A' || c == 'M' || c == 'D' || c.is_digit(10)) && position == 0{
            position += 1;
            dst.push(c);
        }
        if c == '='  && position == 1{
            position += 1;
            f_cmp = true;
        }else if c == ';' && position == 1{
            position += 1;
        }
        if position == 2 && (c.is_uppercase()|| c == '1' || c == '+' || c == '-'|| c =='!' || c == '&' || c == '|'){
            if f_cmp {
                comp.push(c);
            }else {
                jmp.push(c);
            }
        }
        if c =='/'{break;} 
    }

    let mut c_order = String::new();
    let mut bit_decimal = convert_dst_to_num(&dst);
    if f_cmp {
        bit_decimal += convert_cmp_to_num(&comp);
    } else {
        bit_decimal += convert_jmp_to_num(&jmp);
    }
    c_order = to_bit_str(&bit_decimal);
    println!("dst={dst}: comp = {comp}: jmp = {jmp}: bit = {c_order}");

    c_order
    
}
// 4168: 2084,1024,512,256,128,64
fn convert_cmp_to_num(cmp: &String) -> u32{
    //Mの操作する時はa bitを1にする
    let c:[u32;7] = [4096, 2048, 1024, 512, 256, 128, 64];
    let mut decimal:u32 = 0;
    if cmp.len() == 0{
        return decimal;
    }
    let mut flag = false;
    match &cmp[..]{
        "0" => decimal = c[1] + c[3] + c[4],
        "!D" => decimal = c[3] + c[4] + c[6],
        "!A" => decimal = c[1] + c[2] + c[6],
        "!M" => decimal = c[0] + c[1] + c[2] + c[6],
        "D+1" => decimal = c[2] + c[3] + c[4] + c[5] + c[6],
        "A+1" => decimal = c[1] + c[2] + c[4] + c[5] + c[6],
        "M+1" => decimal = c[0] + c[1] + c[2] + c[4] + c[5] + c[6],
        "D-1" => decimal = c[3] + c[4] + c[5],
        "A-1" => decimal = c[1] + c[2] + c[5],
        "M-1" => decimal = c[0] + c[1] + c[2] + c[5],
        "D+A" => decimal = c[5],
        "D+M" => decimal = c[0] + c[5],
        "D-A" => decimal = c[2] + c[5] + c[6],
        "D-M" => decimal = c[0] + c[2] + c[5] + c[6],
        "A-D" => decimal = c[4] + c[5] + c[6],
        "M-D" => decimal = c[0] + c[4] + c[5] + c[6],
        "D&A" => decimal = 0,
        "D&M" => decimal = c[0],
        "D|A" => decimal = c[2] + c[4] + c[6],
        "D|M" => decimal = c[0] + c[2] + c[4] + c[6],
        _ => flag = true,
    }
    if flag {
        match &cmp[..]{
            "1" => decimal = c[1] + c[2] + c[3] + c[4] + c[5] + c[6],
            "-1" => decimal = c[1] + c[2] + c[3] + c[5] + c[6],
            "-D" => decimal = c[3] + c[4] + c[5] + c[6],
            "-A" => decimal = c[1] + c[2] + c[5] + c[6],
            "-M" => decimal = c[0] + c[1] + c[2] + c[5] + c[6],
            "D" => decimal = c[3] + c[4],
            "A" => decimal = c[1] + c[2],
            "M" => decimal = c[0] + c[1] + c[2],
            _=> flag = false,
        }
        if !flag{
            println!("Error: Comp syntax error = {cmp}");
            process::exit(1);
        }
        return decimal;
    }else {
        return decimal;
    }
}


// 不完全
fn convert_jmp_to_num(jmp: &String)->u32{
    let mut decimal:u32 = 0;


    if jmp.len() == 0{
        return(decimal);
    }
    for c in jmp.chars(){
        if c == 'J' || c == 'T' || c == 'Q' {
            continue;
        }else if c == 'G'{
            decimal += 1;
        }else if c == 'E' {
            decimal += 2;
        }else if c == 'L'{
            decimal += 4;
        }else if c == 'N'{
            decimal = 5;
            break;
        }else if c == 'M'{
            decimal = 7;
            break;
        }else {
            println!("Error: JMP syntax error = {c}");
            process::exit(1)
        }
    }
    // 構文チェック必要だけど余裕があればする
    // match &jmp[..]{
    //     "JGT" => decimal+=1,
    //     "JEQ" => decimal
    //     "JMP" => decimal += 7,
    //     _ => println!("error"),
    // }
    decimal
}

fn convert_dst_to_num(str: &String)-> u32{
    let mut decimal:u32 = 0;
    if str.len() == 0{
        return decimal;
    }
    for c in str.chars(){
        if c == 'A' {
            decimal += 8;
        }else if c == 'D'{
            decimal += 16;
        }else if c == 'M'{
            decimal += 32;
        }
    }
    decimal
    

}

fn create_w_file(s: &str)-> Result<File, Error> {
    let  file = s.split('/').fold(Vec::new(), |mut v, i|{
        v.push(i.to_string());
        v
    });
    let len = file.len();
    let  file_name = file[len - 1].split('.').fold(Vec::new(), |mut v, i|{
        v.push(i.to_string());
        v
    });
    let  hack = file_name[0].as_str();
    let file_name = hack.to_string() + ".hack";

    let r_file = File::create(file_name)?;
    Ok(r_file)
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
        if parse_check(writer) {
            // TODO: シンボルテーブルから値をとりだす 
        }else {
            println!("Syntax Error:\n function: convert_atobit \n line: {} ", writer);
            process::exit(1);
        }
    }else {
        // 数字を数値に
        num = writer.parse().unwrap();
    }
    // let mut r_writer = String::new();
    // r_writer = 
    to_bit_str(&num)
}

// A命令の構文チェック
fn parse_check(line: &String) -> bool{
    for c in line.chars(){
        if !c.is_uppercase() && !c.is_lowercase() && !c.is_digit(10){
            if c != '_' && c != '$' && c != ':' && c != '.'{
                return false;
            }
        }
    }
    true
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
        result.push('0');
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
                println!("syntax error :Contains whitespace\n{line}");
                process::exit(1);
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
    command.a = false;
    command.c = false;
    command.l = false;
    // A命令C命令その他を判断するC:dest=comp;jmp '=' or ';'が含まれているかどうかで判断
    let v: Vec<&str> = line.split_whitespace().collect();
    if v.len() == 1 || command.comment{
        if v[0].chars().nth(0).unwrap() == '@'{
            command.a = true;
            return;
        } else if v[0].chars().nth(0).unwrap() == '/'{
            return;
        }
    }
    for c in line.chars() {
        if c == '=' || c == ';' {
            command.c = true;
            return;
        }
    }
    if v[0].chars().nth(0).unwrap() == '(' {
        command.l = true;
        return;
    }
    println!("Syntax Error: non-existent command {line}");
    process::exit(1);
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