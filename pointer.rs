// static GLOBAL: i32 = 99;

struct Point {
    x: i32,
    y: i32,
}

struct Cube {
    width: i32,
    height: i32,
    depth: i32,
}

impl Cube {
    // コンストラクタ
    fn new(width: i32, height: i32, depth: i32) -> Cube {
        Cube {
            width: width,
            height: height,
            depth: depth,
        }
    }

    fn get_num_surfaces() -> i32 {
        6
    }

    fn get_volume(&self) -> i32 {
        self.width * self.height * self.depth
    }

    fn get_area(&self) -> i32 {
        self.width * self.height * 2 + self.height * self.depth * 2 + self.depth * self.width * 2
    }
}

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

    // 配列
    let array = [2, 4, 6, 8, 10];
    for i in 0..5 {
        println!("array[{}] = {}", i, array[i]);
    }

    let array2: [f64; 4] = [1.3, 23.3, 0.0, -55.2];
    for i in 0..4 {
        println!("array[{}] = {}", i, array2[i]);
    }

    // 配列の初期化
    const N: usize = 10;
    let mut array3: [i32; N] = [0; N];
    for i in 0..N {
        array3[i] = -10 * i as i32;
    }
    for i in 0..N {
        println!("array[{}] = {}", i, array3[i]);
    }

    // 2次元配列
    const N2: usize = 3;
    const M2: usize = 5;
    let mut array4 = [[0; M2]; N2];
    for i in 0..N2 {
        for j in 0..M2 {
            array4[i][j] = 10 * i + j + 1;
        }
    }
    for i in 0..N2 {
        for j in 0..M2 {
            println!("array[{}][{}] = {}", i, j, array4[i][j]);
        }
    }

    // 3次元配列
    const N3: usize = 3;
    const M3: usize = 5;
    const L3: usize = 7;
    let mut array5 = [[[0; L3]; M3]; N3];
    for i in 0..N3 {
        for j in 0..M3 {
            for k in 0..L3 {
                array5[i][j][k] = 100 * i + 10 * j + k + 1;
            }
        }
    }
    for i in 0..N3 {
        for j in 0..M3 {
            for k in 0..L3 {
                println!("array[{}][{}][{}] = {}", i, j, k, array5[i][j][k]);
            }
        }
    }

    // 配列と文字列
    let names: [&str; 4] = ["Alice", "Bob", "Chris", "David"];
    for i in 0..4 {
        println!("names[{}] = {}", i, names[i]);
    }

    // 構造体
    let p = Point { x: 10, y: 20 };
    println!("(x, y) = ({}, {})", p.x, p.y);

    let cube = Cube {
        width: 10,
        height: 20,
        depth: 30,
    };
    println!("cube = ({}, {}, {})", cube.width, cube.height, cube.width);
    println!("cube volume = {}", cube.get_volume());
    println!("cube area = {}", cube.get_area());

    let cube2 = Cube::new(10, 20, 50);
    println!(
        "cube2 = ({}, {}, {})",
        cube2.width, cube2.height, cube2.depth
    );
    println!("cube2 surface = {}", Cube::get_num_surfaces());

    // ベクタ型
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(3);
    vec.push(5);

    println!("vec.len() = {}", vec.len());
    println!("vec = {:?}", vec);
    println!("vec.get(1) = {:?}", vec.get(1));
    println!("vec.get(1).unwrap() = {}", vec.get(1).unwrap());

    vec.remove(1);
    println!("rec(after remove) = {:?}", vec);

    // ベクタ型用のマクロ
    let mut vec2: Vec<i32> = vec![1, 3, 5, 7, 9];
    for i in 0..5 {
        vec2[i] = vec2[i] * 2;
    }

    println!("vec2 = {:?}", vec2);

    // String型
    let msg: String = String::from("hello, world.");
    let name = "My name is Alice.".to_string();

    println!("msg = {}", msg);
    println!("name = {}", name);

    // 文字列の結合
    let mut msg2 = String::new();
    msg2.push_str("hello,");
    msg2.push(' ');
    msg2.push_str("world.");

    println!("msg2 = {}", msg2);

    // スライス
    let array5 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let first: &[i32] = &array5[0..5];
    let second = &array5[5..10];

    println!("first = {:?}", first);
    println!("second = {:?}", second);

    // String型とスライス
    let msg3 = String::from("My name is Alice.");
    let slice1 = &msg3[..7];
    let slice2 = &msg3[11..msg3.len()];

    println!("slice1 = {:?}", slice1);
    println!("slice2 = {:?}", slice2);

    // マルチバイト文字列を含むString型
    let ohm = String::from("オーム社");
    let slice = &ohm[..9];

    println!("slice = {:?}", slice);
}
