#![feature(hash_drain_filter)]
#![feature(binary_heap_drain_sorted)]

use log::{debug, trace};

pub mod day1;

pub fn read_input_file(input: &str) -> String {
    let file = std::fs::read_to_string(input)
        .expect("Error while reading provided file name")
        .trim()
        .to_string();
    file
}
