use ahash::AHashSet;
use fnv::FnvHashSet;

pub struct Rucksack<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Rucksack<'a> {
    fn all_items(&self) -> String {
        format!("{}{}", self.left, self.right)
    }
}

pub fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let item_count = line.len();
            let (left, right) = line.split_at(item_count / 2);

            Rucksack { left, right }
        })
        .collect()
}

pub fn part1(input: &[Rucksack]) -> usize {
    let mut item_priority = 0;

    for rucksack in input {
        let item_map: FnvHashSet<char> = rucksack.left.chars().collect();

        let duplicate_value = rucksack
            .right
            .chars()
            .find(|c| item_map.contains(c))
            .expect("No duplicate value found in left and right rucksack pouches");
        if duplicate_value.is_ascii_uppercase() {
            // This is an adjustment from hex values of letters to the given scoring system
            item_priority += (duplicate_value as u8 - 64 + 26) as usize;
        } else if duplicate_value.is_ascii_lowercase() {
            // This is an adjustment from hex values of letters to the given scoring system
            item_priority += (duplicate_value as u8 - 96) as usize;
        }
    }

    item_priority
}

pub fn part1_iter(input: &[Rucksack]) -> usize {
    input
        .iter()
        .map(|rucksack| {
            let item_map: FnvHashSet<char> = rucksack.left.chars().collect();

            let duplicate_value = rucksack
                .right
                .chars()
                .find(|c| item_map.contains(c))
                .expect("No duplicate value found in left and right rucksack pouches");
            if duplicate_value.is_ascii_uppercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 64 + 26) as usize
            } else if duplicate_value.is_ascii_lowercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 96) as usize
            } else {
                unreachable!("Bad data was presented!")
            }
        })
        .sum()
}

pub fn part1_ahash(input: &[Rucksack]) -> usize {
    let mut item_priority = 0;

    for rucksack in input {
        let item_map: AHashSet<char> = rucksack.left.chars().collect();

        let duplicate_value = rucksack
            .right
            .chars()
            .find(|c| item_map.contains(c))
            .expect("No duplicate value found in left and right rucksack pouches");
        if duplicate_value.is_ascii_uppercase() {
            // This is an adjustment from hex values of letters to the given scoring system
            item_priority += (duplicate_value as u8 - 64 + 26) as usize;
        } else if duplicate_value.is_ascii_lowercase() {
            // This is an adjustment from hex values of letters to the given scoring system
            item_priority += (duplicate_value as u8 - 96) as usize;
        }
    }

    item_priority
}

pub fn part1_iter_ahash(input: &[Rucksack]) -> usize {
    input
        .iter()
        .map(|rucksack| {
            let item_map: AHashSet<char> = rucksack.left.chars().collect();

            let duplicate_value = rucksack
                .right
                .chars()
                .find(|c| item_map.contains(c))
                .expect("No duplicate value found in left and right rucksack pouches");
            if duplicate_value.is_ascii_uppercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 64 + 26) as usize
            } else if duplicate_value.is_ascii_lowercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 96) as usize
            } else {
                unreachable!("Bad data was presented!")
            }
        })
        .sum()
}

pub fn part2(input: &[Rucksack]) -> usize {
    input
        .chunks(3)
        .map(|rucksacks| {
            let item_map: FnvHashSet<char> = rucksacks[0].all_items().chars().collect();

            let duplicate_value = rucksacks[1]
                .all_items()
                .chars()
                .filter(|c| item_map.contains(c))
                .find(|&c| rucksacks[2].all_items().contains(c))
                .expect("No triple duplicate value found?");
            if duplicate_value.is_ascii_uppercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 64 + 26) as usize
            } else if duplicate_value.is_ascii_lowercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 96) as usize
            } else {
                unreachable!("Bad data was presented!")
            }
        })
        .sum()
}

pub fn part2_intersection(input: &[Rucksack]) -> usize {
    input
        .chunks(3)
        .map(|rucksacks| {
            let first_item_map: FnvHashSet<char> = rucksacks[0].all_items().chars().collect();
            let second_item_map: FnvHashSet<char> = rucksacks[1].all_items().chars().collect();
            let third_item_map: FnvHashSet<char> = rucksacks[2].all_items().chars().collect();

            let duplicate_value = first_item_map
                .intersection(&second_item_map)
                .map(|c| *c)
                .collect::<FnvHashSet<char>>()
                .intersection(&third_item_map)
                .next()
                .expect("No intersection found")
                .to_owned();
            if duplicate_value.is_ascii_uppercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 64 + 26) as usize
            } else if duplicate_value.is_ascii_lowercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 96) as usize
            } else {
                unreachable!("Bad data was presented!")
            }
        })
        .sum()
}

pub fn part2_ahash(input: &[Rucksack]) -> usize {
    input
        .chunks(3)
        .map(|rucksacks| {
            let item_map: AHashSet<char> = rucksacks[0].all_items().chars().collect();

            let duplicate_value = rucksacks[1]
                .all_items()
                .chars()
                .filter(|c| item_map.contains(c))
                .find(|&c| rucksacks[2].all_items().contains(c))
                .expect("No triple duplicate value found?");
            if duplicate_value.is_ascii_uppercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 64 + 26) as usize
            } else if duplicate_value.is_ascii_lowercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 96) as usize
            } else {
                unreachable!("Bad data was presented!")
            }
        })
        .sum()
}

pub fn part2_intersection_ahash(input: &[Rucksack]) -> usize {
    input
        .chunks(3)
        .map(|rucksacks| {
            let first_item_map: AHashSet<char> = rucksacks[0].all_items().chars().collect();
            let second_item_map: AHashSet<char> = rucksacks[1].all_items().chars().collect();
            let third_item_map: AHashSet<char> = rucksacks[2].all_items().chars().collect();

            let duplicate_value = first_item_map
                .intersection(&second_item_map)
                .map(|c| *c)
                .collect::<AHashSet<char>>()
                .intersection(&third_item_map)
                .next()
                .expect("No intersection found")
                .to_owned();
            if duplicate_value.is_ascii_uppercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 64 + 26) as usize
            } else if duplicate_value.is_ascii_lowercase() {
                // This is an adjustment from hex values of letters to the given scoring system
                (duplicate_value as u8 - 96) as usize
            } else {
                unreachable!("Bad data was presented!")
            }
        })
        .sum()
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

    test!(part1, 157);
    test!(part1_iter, 157);
    test!(part1_ahash, 157);
    test!(part1_iter_ahash, 157);
    test!(part2, 70);
    test!(part2_intersection, 70);
    test!(part2_ahash, 70);
    test!(part2_intersection_ahash, 70);
}
