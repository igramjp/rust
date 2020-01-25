// static GLOBAL: i32 = 99;

fn main() {
    // raw pointer
    // 生ポインタの宣言
    /*
    let x: i32 = 1;
    let ptr: *const i32 = &x;

    unsafe {
        println!("*ptr = {}", *ptr);
    }

    println!("ptr = {:?}", ptr);

    // 生ポインタが指す値の変更
    let mut x: i32 = 1;
    let ptr: *mut i32 = &mut x;

    unsafe {
        println!("before ptr = {:?}, x = {}", ptr, *ptr);

        *ptr = 99;
        println!("after ptr = {:?}, x = {}", ptr, *ptr);
    }

    // 可変ポインタ
    let local: i32 = 10;
    let mut ptr: *const i32 = &local;

    unsafe {
        println!("*ptr = {}", *ptr);
    }
    println!("ptr = {:?}", ptr);

    ptr = &GLOBAL;

    unsafe {
        println!("*ptr = {}", *ptr);
    }
    println!("ptr = {:?}", ptr);
    */

    // reference
    // 参照型（Box型）
    let b: Box<i32> = Box::new(10);
    println!("b = {}", b);

    // 文字列
    let name: &str = "Alice";

    println!("My name is {}.", name);
    println!("name.len() = {}", name.len());
    println!("name.to_uppercase() = {}", name.to_uppercase());

    // 文字コードとマルチバイト文字
    // アルファベット
    let letters = "abcde";
    println!("letters.len() = {}", letters.len());
    println!("UTF8: letters.as_bytes() = {:?}", letters.as_bytes());

    // 漢字
    let kanji = "Ohm社";
    println!("kanji.len() = {}", kanji.len());
    println!("UTF8: kanji.as_bytes() = {:?}", kanji.as_bytes());
}
