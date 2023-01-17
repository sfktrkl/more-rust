use helper::helper::*;
use selection_sort::selection_sort::*;

fn main() {
    println!("\nSelection Sort\n");

    let mut a = [43, 21, 26, 38, 17, 30];
    println!("Initial array: {:?}", a);
    println!("{}: {:?}", selection_sort(&mut a), a);

    println!("\nTry elapsed time with thousand elements.\n");
    sort(selection_sort, &mut create_array());
}
