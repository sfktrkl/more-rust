use counting_sort::counting_sort::*;
use helper::helper::*;

fn main() {
    println!("\nCounting Sort\n");

    let mut a = [43, 21, 26, 38, 17, 30];
    println!("Initial array: {:?}", a);
    println!("{}: {:?}", counting_sort(&mut a), a);

    println!("\nTry elapsed time with thousand elements.");
    println!("Range -1000..1000, which is k, keys\n");
    sort(counting_sort, &mut create_array_range());
}
