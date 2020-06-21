// Loops
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);


    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }


    let my_array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    for x in my_array.iter() {
        println!("Number: {}", x);
    }
}