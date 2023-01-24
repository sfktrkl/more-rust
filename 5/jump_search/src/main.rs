use helper::helper::*;
use jump_search::jump_search::*;

fn main() {
    let value = 30;
    println!("\nJump Search\n");
    search_easy(jump_search, value);

    println!("\nTry elapsed time with 200000 elements.");
    search(jump_search, true, &create_array(true), value);
}
