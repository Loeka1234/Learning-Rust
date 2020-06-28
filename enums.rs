enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

impl Message {
    fn call(&self) {
        println!("Function from Message");
    }
}

fn main() {
    let message = Message::Write(String::from("Hello"));
    message.call();

    let n = Some(5);
    let s = Some("Hello world!");
    let mut absent_number: Option<i32> = None;
    absent_number = Some(100);

    println!("{:?} is some: {}", absent_number, absent_number.is_some());

    println!("New value of absent_number: {:?}", plus_one(absent_number));

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("true");
    } else {
        println!("false");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}