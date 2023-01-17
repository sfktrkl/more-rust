use helper::helper::*;
use insertion_sort::insertion_sort::*;

fn main() {
    println!("\nInsertion Sort\n");

    let mut a = [43, 21, 26, 38, 17, 30];
    println!("Initial array: {:?}", a);
    println!("{}: {:?}", insertion_sort(&mut a), a);
    println!("{}: {:?}", insertion_sort_easy(&mut a), a);

    println!("\nTry elapsed time with thousand elements.\n");
    let array = create_array();
    sort(insertion_sort, &mut array.clone());
    sort(insertion_sort_easy, &mut array.clone());
}
