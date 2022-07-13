fn main(){
    let s = String::from("rust string sample program");
    let v: Vec<&str> = s.split(' ').collect();
    
    let mut result = String::new();
    for i in &v {
        result.push_str(i);
    }
    println!("{result}")
}