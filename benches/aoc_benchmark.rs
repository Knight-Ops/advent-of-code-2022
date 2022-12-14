use advent_of_code_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(target_os = "linux")]
use iai;

macro_rules! bench_please {
    ($lib:ident) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            let input = $lib::input_generator(&raw_input);
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(&input)));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(&input)));
        }
    };
    ($lib:ident, $($func:ident),+) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            let input = $lib::input_generator(&raw_input);
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(&input)));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(&input)));
            $(c.bench_function(&format!("{} {}", stringify!($lib), stringify!($func)), |b| b.iter(|| $lib::$func(&input)));)*
        }
    }
}

macro_rules! bench_please_mut {
    ($lib:ident) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            c.bench_function(&format!("{} part 1 + parse", stringify!($lib)), |b| b.iter(|| $lib::part1(&mut $lib::input_generator(&raw_input))));
            c.bench_function(&format!("{} part 2 + parse", stringify!($lib)), |b| b.iter(|| $lib::part2(&mut $lib::input_generator(&raw_input))));
        }
    };
    ($lib:ident, $($func:ident),+) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2022/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            c.bench_function(&format!("{} part 1 + parse", stringify!($lib)), |b| b.iter(|| $lib::part1(&mut $lib::input_generator(&raw_input))));
            c.bench_function(&format!("{} part 2 + parse", stringify!($lib)), |b| b.iter(|| $lib::part2(&mut $lib::input_generator(&raw_input))));
            $(c.bench_function(&format!("{} {} + parse", stringify!($lib), stringify!($func)), |b| b.iter(|| $lib::$func(&mut $lib::input_generator(&raw_input))));)*
        }
    }
}

bench_please!(day1, part1_iter, part2_binary_heap);
bench_please!(day2, part1_memoized, part2_memoized);
bench_please!(day3, part1_iter, part2_intersection);
bench_please!(day4);
bench_please_mut!(day5);
bench_please!(
    day6,
    part1_skipping,
    part1_hashset,
    part1_skipmap,
    part1_compare,
    part1_lib,
    part2_skipping,
    part2_hashset,
    part2_skipmap,
    part2_compare,
    part2_compare_iter,
    part2_lib
);
bench_please!(day7);
bench_please!(day8, part1_confusing);
bench_please!(day9);
bench_please!(day10);
bench_please_mut!(day11);

criterion_group!(all, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11);
criterion_group!(single, day11);
criterion_main!(single);
