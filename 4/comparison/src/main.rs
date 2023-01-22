use bubble_sort::bubble_sort::*;
use helper::helper::*;
use insertion_sort::insertion_sort::*;
use merge_sort::merge_sort::*;
use quick_sort::quick_sort::*;
use selection_sort::selection_sort::*;

fn main() {
    println!("\nComparison\n");

    println!("Try elapsed time with thousand elements.");
    let array = create_array();
    sort(bubble_sort, &mut array.clone());
    sort(bubble_sort_easy, &mut array.clone());
    sort(selection_sort, &mut array.clone());
    sort(insertion_sort, &mut array.clone());
    sort(insertion_sort_easy, &mut array.clone());
    sort(merge_sort, &mut array.clone());
    sort(merge_sort_easy, &mut array.clone());
    sort(quick_sort, &mut array.clone());
}
