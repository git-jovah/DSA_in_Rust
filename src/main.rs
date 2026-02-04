mod sele_sort;
mod bubble_sort;
mod insertion_sort;

use sele_sort::selection_sort;
use bubble_sort::bubble_sort;
use insertion_sort::insertion_sort;
use sele_sort::_main;

fn main() {
    println!("This is the main");
    selection_sort(&mut vec![34,2,11,67,34,23],"vector");
    bubble_sort(&mut [34,1,6,85,3,4,6],"array");
    insertion_sort(&mut [56,75,4,35,7,9,6,41,3,4],"array");
    _main(); // private functions(_exp_swap) can be called from inside public functions (main)
    sele_sort::_main();
    bubble_sort::main();


}
