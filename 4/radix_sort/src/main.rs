use helper::helper::*;
use radix_sort::radix_sort::*;

fn main() {
    println!("\nCounting Sort\n");

    let mut a = [43, 21, 26, 38, 17, 30];
    println!("Initial array: {:?}", a);
    println!("{}: {:?}", radix_sort(&mut a), a);

    println!("\nTry elapsed time with thousand elements.");
    println!("Range 0..1000, which is k, keys\n");
    sort(radix_sort, &mut create_array_range2());
}
