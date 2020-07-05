#![allow(dead_code)]

enum Example {
    Up(u32, u32), // Tuple in enum
    Down {
        // Struct in enum
        x: u32,
        y: u32,
    },
}

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}

fn main() {
    let u = Direction::Up(Point { x: 0, y: 1 });
    let k = u.match_direction();
    let x = k.destruct();

    println!("{:?}", k);
    println!("Destruct: {}", x);

    let my_int = 10;
    let a = &my_int;
    let ref b = my_int;
    assert_eq!(a, b);
    println!("{}", if a == b { "Equal" } else { "Not equal " });

    let rect = Shape::Rectangle {
        width: 10,
        height: 70,
    };
    let square = Shape::Square(10);
    let circle = Shape::Circle(4.5);

    let area_rect = rect.area();
    println!("Area rectangle: {}", area_rect);

    let area_square = square.area();
    println!("Area square: {}", area_square);

    let area_circle = circle.area();
    println!("Area circle: {}", area_circle);
}
