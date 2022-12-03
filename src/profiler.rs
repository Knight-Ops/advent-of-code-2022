use advent_of_code_2022::*;

fn main() {
    let raw_input = read_input_file(&format!("input/2022/day2.txt"));

    let mut formatted_input = day2::input_generator(&raw_input);

    for _ in 0..10000 {
        let part1 = day2::part1(&mut formatted_input);
        let part2 = day2::part2(&mut formatted_input);
    }
}
