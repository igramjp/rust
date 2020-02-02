// use std::env;
// use std::io;

// use std::fs;
// use std::io;
// use std::io::Write;

// use std::fs::File;
// use std::io::BufRead;
// use std::io::BufReader;

// use std::fs;
// use std::io;
// use std::io::BufWriter;
// use std::io::Write;

use std::fs::OpenOptions;
use std::io;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let mut writer = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("append.txt")
            .unwrap(),
    );

    println!("ファイルに書き込む文字列を入力してください>");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();

    writer.write(s.as_bytes()).unwrap();
    println!("ファイルへの書き込みが終了しました");
    /*
    let mut writer = BufWriter::new(fs::File::create("output2.txt").unwrap());

    println!("ファイルに書き込む文字列を入力してください");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();

    writer.write(s.as_bytes()).unwrap();
    println!("ファイルへの書き込みが終了しました");
    */

    /*
    let mut reader = BufReader::new(File::open("input2.txt").unwrap());

    let mut buff = String::new();
    while reader.read_line(&mut buff).unwrap() > 0 {
        print!("{}", buff);
        buff.clear();
    }
    */

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

    /*
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
    */

    /*
    let s = fs::read_to_string("input1.txt").unwrap();
    println!("input1.txt\n{}", s);
    */

    /*
    let mut file = fs::File::create("output1.txt").unwrap();

    println!("ファイルに書き込む文字列を入力してください>");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();

    let size = file.write(s.as_bytes()).unwrap();
    println!(
        "ファイルへの書き込みが終了しました. bytes = {}",
        size
    );
    */
}
