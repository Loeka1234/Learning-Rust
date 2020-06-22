use std::io::{stdout, stdin, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("Failed to flush");
    stdin().read_line(input)
        .expect("Failed to read");
}

fn main() {
    println!("Calculator");

    loop {
        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut operator = String::new();
    
        println!("Enter the first number: ");
        read(&mut n1);
    
        println!("Enter the second number: ");
        read(&mut n2);
    
        println!("Enter an operator (+-*/): ");
        read(&mut operator);
    
        let n1: f64 = n1.trim().parse().unwrap();
        let n2: f64 = n2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();
    
        let result = match operator {
            '+' => n1 + n2,
            '-' => n1 - n2,
            '*' => n1 * n2,
            '/' => n1 / n2,
            _ => {
                println!("This is not an operator!");
                continue;
            }
        };
    
        println!("{} {} {} = {}", n1, operator, n2, result);
    }
}
