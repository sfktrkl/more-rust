use std::io;

fn main() {
    let mut value = String::new();
    println!("Please enter an integer value: ");
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line.");
    println!("The value you entered is {value}.");
}
