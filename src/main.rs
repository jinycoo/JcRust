#[macro_use] extern crate log;
extern crate env_logger;

mod algorithm;

use algorithm::sorts::bubble;
use algorithm::search::binary;

fn main() {
    env_logger::init().unwrap();
    info!("Application is start.");
    let mut vl = [8, 7, 1, 2, 9, 3, 4, 5, 0, 6];
    bubble::bubble_sort(&mut vl);
    println!("{:?}", vl);
    let vv = [2,5,8,12,45,58,69,75,80,82,83,96,102,112];
    let s = binary::binary_search(&vv, 82);
    println!("{}", s);
}
