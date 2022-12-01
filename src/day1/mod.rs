use super::*;

pub struct ElfList {
    elves: Vec<Elf>
}

impl ElfList {
    fn from_input(input: &str) -> Self {
        ElfList {
            elves: input.split("\n\n").map(|calorie_data| Elf::from_input(calorie_data)).collect(),
        }
    }
}

pub struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn from_input(input: &str) -> Self {
        Elf {
            calories: input.lines().into_iter().map(|num| num.trim().parse::<u32>().expect("Error parsing &str into u32")).collect(),
        }
    }
}

pub fn input_generator(input: &str) -> ElfList {
    ElfList::from_input(input)
}

pub fn part1(input: &ElfList) -> usize {
    let mut max = 0;

    for elf in &input.elves {
        let calories = elf.calories.iter().sum();

        if calories > max {
            max = calories
        }
    }

    max as usize
}

pub fn part1_iter(input: &ElfList) -> usize {
    input.elves.iter().map(|elf| elf.calories.iter().sum::<u32>() as usize).max().unwrap()
}

pub fn part2(input: &ElfList) -> usize {
    let mut calorie_list = vec![0; input.elves.len()];

    input.elves.iter().enumerate().for_each(|(idx, elf)| {
        calorie_list[idx] = elf.calories.iter().sum();
    });

    calorie_list.sort();

    calorie_list.iter().rev().take(3).sum::<u32>() as usize
}

pub fn part2_binary_heap(input: &ElfList) -> usize {
    let mut calorie_list: std::collections::BinaryHeap<u32> = std::collections::BinaryHeap::new();

    input.elves.iter().for_each(|elf| {
        calorie_list.push(elf.calories.iter().sum());
    });

    (calorie_list.pop().unwrap() + calorie_list.pop().unwrap() + calorie_list.pop().unwrap()) as usize
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!("input/2022/{}_test.txt", name[name.len() - 2].trim()));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        }
    }

    test!(part1, 24000);
    test!(part1_iter, 24000);
    test!(part2, 45000);
    test!(part2_binary_heap, 45000);
}