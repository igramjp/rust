use std::char;

fn main() {
    const PI: f64 = 3.14;
    let radius: i32 = 10;

    let cir = 2 as f64 * PI * radius as f64;
    println!("cir = {}", cir);

    let x: i32 = 25;
    let x_string: String = x.to_string();
    println!("x_string = {}", x_string);

    let x_str: &str = &x.to_string();
    println!("x_str = {}", x_str);

    let y: u32 = 5;
    let y_char: char = char::from_digit(y, 10).unwrap();
    println!("y_char = {}", y_char);

    let str1 = "100";
    let str2 = "3.14";

    let num1: i32 = str1.parse().unwrap();
    let num2: f64 = str2.parse().unwrap();

    println!("num1 = {}", num1);
    println!("num2 = {}", num2);
}
