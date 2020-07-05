use std::fmt;

#[derive(Debug)]
struct MyStruct {
    width: u32,
    height: u32,
}

// Methods
impl MyStruct {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area())
    }
}
// Related functions
impl MyStruct {
    fn new(width: u32, height: u32) -> MyStruct {
        MyStruct { width, height }
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}) and Area: {}",
            self.width,
            self.height,
            self.area()
        )
    }
}

fn main() {
    let o = MyStruct::new(35, 55);
    o.show();

    println!("{:?}", o);
    println!("{}", o);
}
