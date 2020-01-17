//! comment for module

/// comment for function
fn main() {
    /*
     * comment
     */
    println!("hello, world."); // comment

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
    */

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
}
