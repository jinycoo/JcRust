mod algorithm;

use algorithm::sorts::bubble;
use algorithm::search::binary;

fn main() {
    bubble::bubble_sort();
    let vv = [2,5,8,12,45,58,69,75,80,82,83,96,102,112];
    let s = binary::binary_search(&vv, 82);
    println!("{}", s);
    println!("Hello, world!");
}
