//! comment for module

/// comment for function

/*
static PI: f64 = 3.14;

fn get_cir(radius: f64) -> f64 {
    2.0 * PI * radius
}

fn get_area(radius: f64) -> f64 {
    PI * radius * radius
}

static mut PI: f64 = 3.14;

fn get_cir(radius: f64) -> f64 {
    unsafe { 2.0 * PI * radius }
}

fn get_area(radius: f64) -> f64 {
    unsafe { PI * radius * radius }
}
*/

macro_rules! max {
    ($x:expr, $y:expr) => {
        if $x >= $y {
            $x
        } else {
            $y
        }
    };
}

fn main() {
    let x = 10;
    let y = 20;
    let z = max!(x, y);

    println!("x = {}, y = {}, max = {}", x, y, z);

    let val: i32 = 200;

    println!("binary: val = {:b}", val);
    println!("octal: val = {:o}", val);
    println!("hexadecimal(lowercase): val = {:x}", val);
    println!("hexadecimal(uppercase): val = {:X}", val);

    let a = 10;
    let b = 20;

    println!("a = {first:x}, b = {second:x}", second = b, first = a);

    let min_i8 = i8::min_value();
    let max_i8 = i8::max_value();

    println!("i8: {} ~ {}", min_i8, max_i8);

    let x_i8: i8 = 0b01001110;
    let y_i8: i8 = x_i8.rotate_left(3);

    println!("y = {:b}", y_i8);

    /*
    let radius = 10.0;

    unsafe {
        PI = 3.0;
    }

    let cir = get_cir(radius);
    let area = get_area(radius);

    println!("cir = {}", cir);
    println!("area = {}", area);

    let radius = 10.0;
    let cir = get_cir(radius);
    let area = get_area(radius);

    println!("cir = {}", cir);
    println!("area = {}", area);
    */

    /*
     * comment
     */
    /*
    println!("hello, world."); // comment
    */

    /*
    let x = 1;
    println!("x = {}", x);

    let mut y = 1;
    println!("y = {}", y);

    y = 100;
    println!("y = {}", y);

    const PI: i32 = 3;
    let radius = 10;

    let cir = 2 * PI * radius;
    println!("cir = {}", cir);

    let area = PI * radius * radius;
    println!("area = {}", area);

    let x = 10;
    let y = 3;
    let mut z;

    z = x + y;
    println!("{} + {} = {}", x, y, z);

    z = x - y;
    println!("{} - {} = {}", x, y, z);

    z = x * y;
    println!("{} * {} = {}", x, y, z);

    z = x / y;
    println!("{} / {} = {}", x, y, z);

    z = x % y;
    println!("{} % {} = {}", x, y, z);

    let x = (10 + 3) * 5 - (30 - 4) / 2;
    println!("x = {}", x);

    let x: i32 = 100000000;
    println!("x = {}", x);

    let dec = 25;
    let bin = 0b11001;
    let oct = 0o31;
    let hex = 0x19;

    println!("dec = {}", dec);
    println!("bin = {}", bin);
    println!("oct = {}", oct);
    println!("hex = {}", hex);

    let a: f64 = 1000.0;
    let b: f64 = 33.0;
    let c = 2.5;

    let x = a / b;
    let y = b / a;
    let z = a / c;

    println!("{} / {} = {}", a, b, x);
    println!("{} / {} = {}", b, a, y);
    println!("{} / {} = {}", a, c, z);

    let x: bool = true;
    let y = false;

    println!("x = {}", x);
    println!("y = {}", y);

    let c1: char = 'O';
    let c2: char = 'h';
    let c3: char = 'm';
    let c4: char = '\n';
    let c5 = 'R';
    let c6 = 'u';
    let c7 = 's';
    let c8 = 't';

    println!("{}{}{}{}{}{}{}{}", c1, c2, c3, c4, c5, c6, c7, c8);

    let p = (10, 25);
    println!("(x, y) = ({}, {})", p.0, p.1);

    let q = (5, 10, 30);
    println!("(x, y, z) = ({}, {}, {})", q.0, q.1, q.2);

    let s = (80, 90, 85, true);
    let (math, english, verbal, result) = s;
    println!(
        "(math, english, verbal, result) = ({}, {}, {}, {})",
        math, english, verbal, result
    );

    let (_, _, _, result2) = s;
    println!("result = {}", result2);

    let x = 20;

    if x > 10 {
        println!("x = {}", x);
        println!("xの値は10より大きいです");
    }

    if (x + 30) >= 35 {
        println!("x = {}", x);
        println!("x+30の値は35以上です");
    }

    if true {
        println!("条件が真なので必ず実行されます");
    }

    if x > 10 && x < 30 {
        println!("x = {}", x);
        println!("xの値は10より大きい，かつ30より小さいです");
    }

    if x <= 10 || x >= 30 {
        println!("x = {}", x);
        println!("xの値は10以下，または30以上です");
    }

    if !(x < 0) {
        println!("x = {}", x);
        println!("xは非負の値です");
    }

    let y = 5;

    if y > 10 {
        println!("x = {}", x);
        println!("xの値は10より大きいです");
    } else {
        println!("x = {}", x);
        println!("xの値は10以下です");
    }

    let score = 85;

    if score >= 90 {
        println!("成績はAです");
    } else if score >= 80 {
        println!("成績はBです");
    } else if score >= 70 {
        println!("成績はCです");
    } else if score >= 60 {
        println!("成績はDです");
    } else {
        println!("成績はEです");
    }

    for i in 0..10 {
        println!("i = {}", i);
    }

    for i in 0..5 {
        for j in 0..5 {
            println!("(i,j) = ({},{})", i, j);
        }
    }

    let mut i = 0;
    while i < 10 {
        println!("i = {}", i);
        i += 1;
    }

    let mut i = 0;
    loop {
        println!("i = {}", i);
        i += 1;

        if i == 9 {
            break;
        }
    }

    println!("user-defined function begin");
    my_func();
    add(10, 5);
    let z = add_return(10, 5);
    println!("10 + 5 = {}", z);
    println!("user-defined function end");

    let mut counter = 0;
    for i in 0..10 {
        counter = incr(counter);
        println!("loop i = {} : counter = {}", i, counter);
    }

    let x = 10;
    let y = 30;

    let point = get_point(x, y);
    println!("point = ({}, {})", point.0, point.1);

    let var = factorial(5);
    println!("factorial of 5 = {}", var);

    let mut x = 10;

    if x < 0 {
        let y = x + 1;
        x = x + 1;
        println!("y = {}", y);
    } else {
        let z = x - 1;
        x = x - 1;
        println!("z = {}", z);
    }

    println!("x = {}", x);

    let mut x = 0;

    for i in 0..3 {
        for j in 0..3 {
            let y = i * 10 + j;
            x = x + 1;
            println!("y = {}", y);
        }

        let j = 'a';
        println!("j = {}", j);
    }

    println!("x = {}", x);

    let mut x: i32 = 10;

    func(x);

    x = x / 2;
    println!("main: x = {}", x);
    */
}

/*
fn my_func() {
    println!("hello, world.");
}

fn add(x: i32, y: i32) {
    let z = x + y;
    println!("add({}, {}) = {}", x, y, z);
}

fn add_return(x: i32, y: i32) -> i32 {
    x + y
}

fn incr(cnt: i32) -> i32 {
    if cnt >= 8 {
        println!("reset counter value");
        return 1;
    } else {
        println!("add 1 counter value");
        cnt + 1
    }
}

fn get_point(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn factorial(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else {
        factorial(n - 1) * n
    }
}

fn func(mut x: i32) {
    x = x * 2;
    println!("func: x = {}", x);
}
*/
