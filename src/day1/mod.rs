use super::*;

#[derive(Debug)]
pub struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn new(calories: &[u32]) -> Self {
        Elf {
            calories: calories.to_vec(),
        }
    }
}

pub fn input_generator(input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut current_calories = Vec::new();
    input.lines().for_each(|line| {
        if line.is_empty() {
            let e = Elf::new(&current_calories);
            elves.push(e);
            current_calories.clear();
        } else {
            current_calories.push(line.parse::<u32>().unwrap());
        }
    });

    if !current_calories.is_empty() {
        let e = Elf::new(&current_calories);
        elves.push(e);
        current_calories.clear();
    }

    elves
}

pub fn part1(input: &[Elf]) -> usize {
    let mut max = 0;

    for elf in input {
        let calories = elf.calories.iter().sum();

        if calories > max {
            max = calories
        }
    }

    max as usize
}

pub fn part1_iter(input: &[Elf]) -> usize {
    input
        .iter()
        .map(|elf| elf.calories.iter().sum::<u32>() as usize)
        .max()
        .unwrap()
}

pub fn part2(input: &[Elf]) -> usize {
    let mut calorie_list = vec![0; input.len()];

    input.iter().enumerate().for_each(|(idx, elf)| {
        calorie_list[idx] = elf.calories.iter().sum();
    });

    calorie_list.sort();

    calorie_list.iter().rev().take(3).sum::<u32>() as usize
}

pub fn part2_binary_heap(input: &[Elf]) -> usize {
    let mut calorie_list: std::collections::BinaryHeap<u32> = std::collections::BinaryHeap::new();

    input.iter().for_each(|elf| {
        calorie_list.push(elf.calories.iter().sum());
    });

    (calorie_list.pop().unwrap() + calorie_list.pop().unwrap() + calorie_list.pop().unwrap())
        as usize
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2022/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    test!(part1, 24000);
    test!(part1_iter, 24000);
    test!(part2, 45000);
    test!(part2_binary_heap, 45000);
}
