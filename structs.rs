struct User {
    username: String,
    password: String,
    sign_in_count: u64,
    active: bool
}

// Tuple Struct
struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("Loeka"),
        password: String::from("LoekaPass"),
        sign_in_count: 5,
        active: true
    };

    let user2 = build_user(String::from("Username"), String::from("Password"));

    let user3 = User {
        username: String::from("Loeka2"),
        password: String::from("Loeka2Pass"),
        ..user1
    };

    println!("{}, {}, {}, {}",user3.username,user3.password,user3.sign_in_count,user3.active);

    // Tuple struct
    let black = Color(0, 0, 0);
    let point = Point(1131, 1133, 931);
    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", point.0, point.1, point.2);
}

fn build_user(username: String, password: String) -> User {
    User {
        username,
        password,
        sign_in_count: 0,
        active: true
    }
}