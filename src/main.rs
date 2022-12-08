use advent_of_code_2022::*;

use log::debug;
use std::alloc::{GlobalAlloc, Layout, System};

use logging_allocator::LoggingAllocator;

macro_rules! run {
    ($lib:ident) => {
        {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));

            let formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let part2 = $lib::part2(&formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);
        }
    };
    ($lib:ident, $($func:ident), +) => {
        {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));

            let formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let part2 = $lib::part2(&formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);

            $(println!("Solution for {} {} : {}", stringify!($lib), stringify!($func), $lib::$func(&formatted_input));)*
        }
    }
}

macro_rules! run_mut {
    ($lib:ident) => {
        {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));

            let mut formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&mut formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let mut formatted_input = $lib::input_generator(&raw_input);

            let part2 = $lib::part2(&mut formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);
        }
    };
    ($lib:ident, $($func:ident), +) => {
        {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));

            let mut formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&mut formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let mut formatted_input = $lib::input_generator(&raw_input);

            let part2 = $lib::part2(&mut formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);

            $(println!("Solution for {} {} : {}", stringify!($lib), stringify!($func), $lib::$func(&$lib::input_generator(&raw_input)));)*
        }
    }
}

// #[global_allocator]
// static GLOBAL: LoggingAllocator = LoggingAllocator::new();

use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    // GLOBAL.enable_logging();

    run!(day1);
    run!(day2);
    run!(day3);
    run!(day4);
    run_mut!(day5);
    run!(day6);
    run!(day7);
    run!(day8);
}
