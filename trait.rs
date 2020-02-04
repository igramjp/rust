use std::fmt;

trait HasArea {
    fn get_area(&self) -> i32;
}
struct Rectangle {
    width: i32,
    height: i32,
}

impl HasArea for Rectangle {
    fn get_area(&self) -> i32 {
        self.width * self.height
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} * {}", self.width, self.height)
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width = {} and height = {}", self.width, self.height)
    }
}

struct Triangle {
    base: i32,
    attitude: i32,
}

impl HasArea for Triangle {
    fn get_area(&self) -> i32 {
        self.base * self.attitude / 2
    }
}

trait Number {
    fn display_all(&self);
}

impl Number for i32 {
    fn display_all(&self) {
        println!("{}, {:b}, {:o}, {:x}", &self, &self, &self, &self);
    }
}

struct Student {
    name: String,
}

trait Message {
    fn to_message(&self);
}

impl Message for Student {
    fn to_message(&self) {
        println!("私の名前は{}です", self.name);
    }
}

trait EnglishMessage: Message {
    fn to_english_message(&self);
}

impl EnglishMessage for Student {
    fn to_english_message(&self) {
        println!("My name is {}.", self.name);
    }
}

trait Polygon {
    fn get_area2(&self) -> i32;

    // Default Method
    fn is_square(&self) -> bool {
        true
    }
}

impl Polygon for Rectangle {
    fn get_area2(&self) -> i32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        if self.width == self.height {
            true
        } else {
            false
        }
    }
}

struct Square {
    edge: i32,
}

impl Polygon for Square {
    fn get_area2(&self) -> i32 {
        self.edge * self.edge
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    let tri = Triangle {
        base: 10,
        attitude: 30,
    };

    println!("長方形の面積 = {}", rect.get_area());
    println!("三角形の面積 = {}", tri.get_area());

    println!("rect: {}", rect);
    println!("rect: {:?}", rect);

    let x: i32 = 25;
    x.display_all();

    100.display_all();

    let student = Student {
        name: "Alice".to_string(),
    };

    student.to_message();
    student.to_english_message();

    let sq = Square { edge: 15 };
    println!(
        "rect: 面積 = {}, 正方形か否か = {}",
        rect.get_area2(),
        rect.is_square()
    );
    println!(
        "sq: 面積 = {}, 正方形か否か = {}",
        sq.get_area2(),
        sq.is_square()
    );
}
