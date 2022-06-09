mod bar;
mod sorts;

use bar::{get_bars, output_bars};
use sorts::{bubble_sort, insertion_sort};

fn main() {
    println!("{}c", 27 as char);
    let mut user_choice = String::new();
    println!("1) Bubble Sort\n2) Insertion Sort\n\n");
    std::io::stdin().read_line(&mut user_choice).expect("Could not read stdin");

    let start_vec = vec![3,8,7,4,1,8,2,6,4,3,7,6];
    let mut bars = get_bars(&start_vec);

    if user_choice.trim() == "1" {
        bubble_sort(&mut bars);
    } else if user_choice.trim() == "2" {
        insertion_sort(&mut bars);
    }

    
}
