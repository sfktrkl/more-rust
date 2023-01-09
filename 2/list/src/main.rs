use list::list::*;

fn print<T: std::fmt::Debug + PartialEq>(list: &List<T>) {
    let mut index = 0;
    while index < list.count() {
        print!("{:?} ", list.get(index).unwrap());
        index += 1;
    }
    print!("\n");
}

fn search<T: std::fmt::Debug + PartialEq>(list: &List<T>, value: T) {
    let found = list.search(value);
    match found {
        usize::MAX => println!("71 is not found!"),
        _ => println!("71 found at index {:?}", found),
    }
}

fn main() {
    let mut list = List::<i32>::new();
    list.insert(0, 23);
    list.insert(1, 47);
    list.insert(2, 87);
    list.insert(3, 35);
    list.insert(4, 92);

    println!("\nList items!");
    print(&list);

    list.insert(2, 25);
    list.insert(2, 71);
    println!("\nItems inserted!");
    print(&list);

    println!("\nSearch item 71");
    search(&list, 71);

    list.remove(2);
    println!("\nItem removed!");
    print(&list);

    println!("\nSearch item 71 again");
    search(&list, 71);
}
