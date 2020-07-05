use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);

    for (key, value) in &hm {
        println!("{}: {}", key, value);
    }

    match hm.get("random") {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    hm.remove("strings");
    println!("{:?}", &hm);
}
