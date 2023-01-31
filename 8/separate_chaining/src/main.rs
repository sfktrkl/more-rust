use hash_table::hash_table::*;

fn main() {
    println!("Hash Table - Separate Chaining");
    let mut hash_table = HashTable::<u64, String>::new();

    println!("\nIs empty: {:?}", hash_table.is_empty());
    hash_table.print();

    println!("\nInsert 434-Dylan");
    hash_table.insert(434, String::from("Dylan"));
    hash_table.print();
    println!("Is empty: {:?}", hash_table.is_empty());

    hash_table.insert(391, String::from("Dominic"));
    hash_table.insert(806, String::from("Adam"));
    hash_table.insert(117, String::from("Lindsey"));
    hash_table.insert(548, String::from("Cameron"));
    hash_table.insert(669, String::from("Terry"));
    hash_table.insert(722, String::from("Brynn"));
    hash_table.insert(276, String::from("Jody"));
    hash_table.insert(953, String::from("Frankie"));
    hash_table.insert(895, String::from("Vanessa"));

    println!("\nSearch value for 669: {:?}", hash_table.search(669));
    hash_table.print();

    println!("\nRemove 117-Lindsey");
    hash_table.remove(117);
    println!("Search value for 117: {:?}", hash_table.search(117));

    hash_table.print();
}
