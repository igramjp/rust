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
    */

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
}
