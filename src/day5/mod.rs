use std::collections::VecDeque;

use ahash::AHashMap;

#[derive(Debug)]
pub struct Supply {
    stacks: AHashMap<usize, VecDeque<char>>,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

pub fn input_generator(input: &str) -> Supply {
    let mut image = input.split("\n\n");

    let layout_string = image
        .next()
        .expect("Image wasn't split properly, couldn't get layout");

    let mut stacks: AHashMap<usize, VecDeque<char>> = AHashMap::new();
    layout_string.lines().for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .for_each(|(idx, c)| {
                // This magical 4 is based on each crate being in the format of [X] with a space following it, so we can just leverage that to find its position in the crate list
                let calculated_stack = (idx / 4) + 1;

                if let Some(stack) = stacks.get_mut(&calculated_stack) {
                    stack.push_back(c);
                } else {
                    stacks.insert(calculated_stack, VecDeque::from([c; 1]));
                }
            })
    });

    let instruction_string = image
        .next()
        .expect("Image wasn't split properly, couldn't get instructions");

    let instructions = instruction_string
        .lines()
        .map(|line| {
            let mut instructions = line.split(" ").filter_map(|str| str.parse::<usize>().ok());
            Instruction {
                count: instructions
                    .next()
                    .expect("Error parsing instruction, could not get count"),
                from: instructions
                    .next()
                    .expect("Error parsing instruction, could not get from"),
                to: instructions
                    .next()
                    .expect("Error parsing instruction, could not get to"),
            }
        })
        .collect::<Vec<Instruction>>();

    Supply {
        stacks,
        instructions,
    }
}

pub fn part1(input: &mut Supply) -> String {
    input.instructions.iter().for_each(|instr| {
        (0..instr.count).for_each(|_| {
            let elf_crate = input.stacks.get_mut(&instr.from).expect("Error while getting from stack").pop_front().expect("Tried to move crate that doesn't exist");
            input.stacks.get_mut(&instr.to).expect("Error while getting to stack").push_front(elf_crate);
        })
    });

    let mut final_top_crates = String::with_capacity(input.stacks.len());
    for x in 1..=input.stacks.len() {
        final_top_crates.push(*input.stacks.get(&x).expect("Error construction final crate list").front().expect("Error retrieving crate from empty stack"));
    }

    final_top_crates
}

pub fn part2(input: &mut Supply) -> String {
    input.instructions.iter().for_each(|instr| {
        let elf_crate = input.stacks.get_mut(&instr.from).expect("Error while getting from stack").drain(0..instr.count).rev().collect::<String>();
        elf_crate.chars().for_each(|c| {
            input.stacks.get_mut(&instr.to).expect("Error while getting to stack").push_front(c);
        })
    });

    let mut final_top_crates = String::with_capacity(input.stacks.len());
    for x in 1..=input.stacks.len() {
        final_top_crates.push(*input.stacks.get(&x).expect("Error construction final crate list").front().expect("Error retrieving crate from empty stack"));
    }

    final_top_crates
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

    macro_rules! test_mut {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2022/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let mut input = super::input_generator(&i);
                assert_eq!(super::$func(&mut input), $val);
            }
        };
    }

    test_mut!(part1, "CMZ");
    test_mut!(part2, "MCD");
}
