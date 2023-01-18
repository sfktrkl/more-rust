use helper::helper::*;
use merge_sort::merge_sort::*;

fn main() {
    println!("\nMerge Sort\n");

    let mut a = [43, 21, 26, 38, 17, 30];
    println!("Initial array: {:?}", a);
    println!("{}: {:?}", merge_sort(&mut a), a);
    println!("{}: {:?}", merge_sort_easy(&mut a), a);

    println!("\nTry elapsed time with thousand elements.\n");
    let array = create_array();
    sort(merge_sort, &mut array.clone());
    sort(merge_sort_easy, &mut array.clone());
}
