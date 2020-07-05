fn main() {
    let s = Some('c');
    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }
    // Above same as below
    if let Some(i) = s {
        println!("{}", i);
    }

    let mut i = Some(0);

    loop {
        match i {
            Some(j) => {
                if j > 19 {
                    println!("Quit");
                    i = None;
                } else {
                    println!("{}", j);
                    i = Some(j + 2);
                }
            }
            _ => {
                break;
            }
        }
    }
    // Above same as below
    while let Some(j) = i {
        if j > 19 {
            println!("Quit");
            i = None;
        } else {
            println!("{}", j);
            i = Some(j + 2);
        }
    }
}
