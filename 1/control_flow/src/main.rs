use std::io;

fn main() {
    let mut value = String::new();
    println!("Please enter an integer value: ");
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line.");
    let number: i32 = value.trim().parse().expect("Input is not an integer.");

    if number > 100 {
        println!("Greater than 100");
    } else if number < 100 {
        println!("Less than 100");
    } else {
        println!("Equals to 100");
    }

    match number {
        10 => println!("Ten"),
        100 => println!("Hundred"),
        _ => println!("Other"),
    }
}
