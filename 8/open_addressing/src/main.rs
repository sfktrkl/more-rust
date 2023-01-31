use hash_table::hash_table::*;

fn main() {
    println!("Hash Table - Open Addressing");
    let mut hash_table = HashTable::<u64, String>::new();

    println!("\nIs empty: {:?}", hash_table.is_empty());
    hash_table.print();

    println!("\nInsert 541-Cameron, should be in Cell 1");
    hash_table.insert(541, String::from("Cameron"));
    hash_table.print();
    println!("Is empty: {:?}", hash_table.is_empty());

    println!("\nInsert 669-Terry, should be in Cell 6");
    hash_table.insert(669, String::from("Terry"));
    hash_table.print();

    println!("\nInsert Others");
    hash_table.insert(276, String::from("Jody"));
    hash_table.insert(806, String::from("Adam"));
    hash_table.insert(434, String::from("Dylan"));
    hash_table.insert(117, String::from("Lindsey"));
    hash_table.insert(722, String::from("Brynn"));
    hash_table.print();

    println!("\nInsert More, nothing happens");
    hash_table.insert(419, String::from("James"));
    hash_table.insert(687, String::from("Michael"));
    hash_table.print();

    println!("\nSearch value for 541: {:?}", hash_table.search(541));
    hash_table.print();

    println!("\nRemove 541-Cameron and 669-Terry");
    hash_table.remove(541);
    hash_table.remove(669);
    hash_table.print();

    println!("\nInsert 975-Linda, should be in Cell 0");
    hash_table.insert(975, String::from("Linda"));
    hash_table.print();

    println!("\nInsert 541-Cameron, should be in Cell 6 this time");
    hash_table.insert(541, String::from("Cameron"));
    hash_table.print();

    println!("\nRemove 541-Cameron");
    hash_table.remove(541);
    hash_table.print();
}
