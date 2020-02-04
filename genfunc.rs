use std::cmp::PartialOrd;
use std::ops::Mul;

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T>
where
    T: Mul<Output = T> + Copy,
{
    fn get_area(&self) -> T {
        self.width * self.height
    }
}

struct Rectangle2<T, U> {
    width: T,
    height: U,
}

fn main() {
    let x: Option<i32> = Some(10);
    let y: Option<f64> = Some(-0.55);

    println!("x = {:?}", x);
    println!("y = {}", y.unwrap());

    let intmax = max(10, 3);
    println!("大きい方の整数 = {}", intmax);

    let realmax = max(1.55, -3.2);
    println!("大きい方の実数 = {}", realmax);

    drop(x);
    drop(y);

    let x = multiply(10, 3);
    let y = multiply(5.1, 3.3);

    println!("x = {}", x);
    println!("y = {}", y);

    let max = absmax(10.5, -30.1);
    println!("大きい方の値 = {}", max);

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 5.5,
        height: 22.3,
    };

    println!("rect1: width = {}, height = {}", rect1.width, rect1.height);
    println!("rect2: width = {}, height = {}", rect2.width, rect2.height);

    let rect3 = Rectangle2 {
        width: 10,
        height: 20.5,
    };
    let rect4 = Rectangle2 {
        width: 5.5,
        height: 22.3,
    };

    println!("rect3: width = {}, height = {}", rect3.width, rect3.height);
    println!("rect4: width = {}, height = {}", rect4.width, rect4.height);

    println!("rect1の面積 = {}", rect1.get_area());
    println!("rect2の面積 = {}", rect2.get_area());
}

fn max<T: std::cmp::PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x
    } else {
        y
    }
}

fn multiply<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

fn absmax<T>(x: T, y: T) -> T
where
    T: Mul<Output = T> + PartialOrd + Copy,
{
    let sq_x = x * x;
    let sq_y = y * y;
    if sq_x >= sq_y {
        x
    } else {
        y
    }
}
