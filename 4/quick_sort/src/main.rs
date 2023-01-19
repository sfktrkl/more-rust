use helper::helper::*;
use quick_sort::quick_sort::*;

fn main() {
    println!("\nQuick Sort\n");

    let mut a = [43, 21, 26, 38, 17, 30];
    println!("Initial array: {:?}", a);
    println!("{}: {:?}", quick_sort(&mut a), a);

    println!("\nTry elapsed time with thousand elements.\n");
    sort(quick_sort, &mut create_array());
}
