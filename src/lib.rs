#![feature(hash_drain_filter)]
#![feature(binary_heap_drain_sorted)]
#![feature(cell_update)]

use anyhow::{anyhow, Result};
use log::{debug, trace};

pub mod communication_system;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;

pub fn read_input_file(input: &str) -> String {
    let file = std::fs::read_to_string(input)
        .expect("Error while reading provided file name")
        .to_string();
    file
}
