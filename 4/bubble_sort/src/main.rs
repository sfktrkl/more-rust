use bubble_sort::bubble_sort::*;
use helper::helper::*;

fn main() {
    println!("\nBubble Sort\n");

    let mut a = [43, 21, 26, 38, 17, 30];
    let mut b = a.clone();
    println!("Initial array: {:?}", a);
    println!("{}: {:?}", bubble_sort(&mut a), a);
    println!("{}: {:?}", bubble_sort_easy(&mut b), b);

    println!("\nTry elapsed time with thousand elements.\n");
    let array = create_array();
    sort(bubble_sort, &mut array.clone());
    sort(bubble_sort_easy, &mut array.clone());
}
