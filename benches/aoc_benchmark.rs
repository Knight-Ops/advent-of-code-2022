use advent_of_code_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

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

criterion_group!(
    all, day1
);
criterion_group!(single, day1);
criterion_main!(single);
