fn main() {
    let mut vector = Vec::<i32>::new();
    vector.push(35);
    vector.push(41);
    vector.push(94);

    println!("Vector: {:?}", vector);

    let i = vector[0];
    println!("Item at position 0: {}", i);

    match vector.iter_mut().find(|&&mut x| x == 41) {
        Some(value) => {
            println!("41 found in the vector.");
            *value = 57;
        }
        _ => println!("41 not found in the vector."),
    }

    println!("Vector: {:?}", vector);

    match vector.iter().position(|&x| x == 57) {
        Some(value) => println!("57 found in vector at position: {}", value),
        _ => println!("57 not found in the vector."),
    }
}
