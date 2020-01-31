use std::env;
// use std::io;

fn main() {
    /*
    println!("Please input string >");

    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();

    println!("input string = {}", s);
    */

    /*
    println!("1:氏名，2:国語，3:数学，4:英語，の点数を入力してください >");

    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();

    let strs: Vec<&str> = line.split_whitespace().collect();
    let name = strs[0];
    let verval: i32 = strs[1].parse().unwrap();
    let math: i32 = strs[2].parse().unwrap();
    let english: i32 = strs[3].parse().unwrap();
    let sum: i32 = verval + math + english;

    println!("{}の合計点 = {}", name, sum);
    */

    /*
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    */

    let args: Vec<String> = env::args().collect();

    let op: &str = args[1].as_str();
    let x: i32 = args[2].parse().unwrap();
    let y: i32 = args[3].parse().unwrap();

    if op == "add" {
        println!("{} + {} = {}", x, y, x + y);
    } else if op == "sub" {
        println!("{} - {} = {}", x, y, x - y);
    } else if op == "mult" {
        println!("{} * {} = {}", x, y, x * y);
    } else if op == "div" {
        println!("{} / {} = {}", x, y, x / y);
    } else {
        println!("演算子が不正です");
    }
}
