use fnv::FnvHashMap;
use fnv::FnvHashSet;

use crate::communication_system;

pub fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn part1(input: &[char]) -> usize {
    let mut working_window: Vec<char> = Vec::with_capacity(4);
    let (location, _) = input
        .windows(4)
        .enumerate()
        .filter(|(_, window)| {
            window[..].clone_into(&mut working_window);

            working_window.sort_unstable();

            for doubles in working_window.windows(2) {
                if doubles[0] == doubles[1] {
                    return false;
                }
            }

            return true;
        })
        .next()
        .expect("No window passed the filter");

    location + 4
}

pub fn part1_skipping(input: &[char]) -> usize {
    const WINDOW_SIZE: usize = 4;

    let mut start_idx = 0;

    let mut working_window: Vec<char> = Vec::with_capacity(WINDOW_SIZE);
    'outer: while start_idx < input.len() {
        input[start_idx..start_idx + WINDOW_SIZE].clone_into(&mut working_window);

        working_window.sort_unstable();

        for doubles in working_window.windows(2) {
            if doubles[0] == doubles[1] {
                let (skip_idx, _) = input[start_idx..start_idx + WINDOW_SIZE]
                    .iter()
                    .enumerate()
                    .find(|(_, &x)| x == doubles[0])
                    .unwrap();
                start_idx += skip_idx + 1;
                continue 'outer;
            }
        }

        return start_idx + WINDOW_SIZE;
    }

    unreachable!("Input provided is 0 bytes long")
}

pub fn part1_hashset(input: &[char]) -> usize {
    let mut set = FnvHashSet::default();
    let (location, _) = input
        .windows(4)
        .enumerate()
        .filter(|(_, window)| {
            set.clear();

            for c in window.iter() {
                if !set.insert(c) {
                    return false;
                }
            }

            return true;
        })
        .next()
        .expect("No window passed the filter");

    location + 4
}

pub fn part1_skipmap(input: &[char]) -> usize {
    const WINDOW_SIZE: usize = 4;

    let mut start_idx = 0;

    let mut hm = FnvHashMap::default();
    'outer: while start_idx < input.len() {
        hm.clear();

        for (idx, c) in input[start_idx..start_idx + WINDOW_SIZE].iter().enumerate() {
            if let Some(duplicate_idx) = hm.insert(c, idx) {
                start_idx += duplicate_idx + 1;
                continue 'outer;
            }
        }
        return start_idx + WINDOW_SIZE;
    }

    unreachable!("Input provided is 0 bytes long")
}

pub fn part1_compare(input: &[char]) -> usize {
    const WINDOW_SIZE: usize = 4;
    let mut start_idx = 0;

    'outer: while start_idx < input.len() {
        for (idx, c) in input[start_idx..start_idx + WINDOW_SIZE].iter().enumerate() {
            for cc in input[start_idx + idx + 1..start_idx + WINDOW_SIZE].iter() {
                if c == cc {
                    start_idx += idx + 1;
                    continue 'outer;
                }
            }
        }
        return start_idx + WINDOW_SIZE;
    }

    unreachable!("Input provided is 0 bytes long")
}
pub fn part1_lib(input: &[char]) -> usize {
    let handheld = communication_system::Device::from(input);
    handheld.find_start_of_packet()
}

pub fn part2(input: &[char]) -> usize {
    let mut working_window: Vec<char> = Vec::with_capacity(14);
    let (location, _) = input
        .windows(14)
        .enumerate()
        .filter(|(_, window)| {
            window[..].clone_into(&mut working_window);

            working_window.sort_unstable();

            for doubles in working_window.windows(2) {
                if doubles[0] == doubles[1] {
                    return false;
                }
            }

            return true;
        })
        .next()
        .expect("No window passed the filter");

    location + 14
}

pub fn part2_skipping(input: &[char]) -> usize {
    const WINDOW_SIZE: usize = 14;

    let mut start_idx = 0;

    let mut working_window: Vec<char> = Vec::with_capacity(WINDOW_SIZE);
    'outer: while start_idx < input.len() {
        input[start_idx..start_idx + WINDOW_SIZE].clone_into(&mut working_window);

        working_window.sort_unstable();

        for doubles in working_window.windows(2) {
            if doubles[0] == doubles[1] {
                let (skip_idx, _) = input[start_idx..start_idx + WINDOW_SIZE]
                    .iter()
                    .enumerate()
                    .find(|(_, &x)| x == doubles[0])
                    .unwrap();
                start_idx += skip_idx + 1;
                continue 'outer;
            }
        }

        return start_idx + WINDOW_SIZE;
    }

    unreachable!("Input provided is 0 bytes long")
}

pub fn part2_hashset(input: &[char]) -> usize {
    let mut set = FnvHashSet::default();
    let (location, _) = input
        .windows(14)
        .enumerate()
        .filter(|(_, window)| {
            set.clear();

            for c in window.iter() {
                if !set.insert(c) {
                    return false;
                }
            }

            return true;
        })
        .next()
        .expect("No window passed the filter");

    location + 14
}

pub fn part2_skipmap(input: &[char]) -> usize {
    const WINDOW_SIZE: usize = 14;

    let mut start_idx = 0;

    let mut hm = FnvHashMap::default();
    'outer: while start_idx < input.len() {
        hm.clear();

        for (idx, c) in input[start_idx..start_idx + WINDOW_SIZE].iter().enumerate() {
            if let Some(duplicate_idx) = hm.insert(c, idx) {
                start_idx += duplicate_idx + 1;
                continue 'outer;
            }
        }
        return start_idx + WINDOW_SIZE;
    }

    unreachable!("Input provided is 0 bytes long")
}

pub fn part2_compare(input: &[char]) -> usize {
    const WINDOW_SIZE: usize = 14;
    let mut start_idx = 0;

    'outer: while start_idx < input.len() {
        for (idx, c) in input[start_idx..start_idx + WINDOW_SIZE].iter().enumerate() {
            for cc in input[start_idx + idx + 1..start_idx + WINDOW_SIZE].iter() {
                if c == cc {
                    start_idx += idx + 1;
                    continue 'outer;
                }
            }
        }
        return start_idx + WINDOW_SIZE;
    }

    unreachable!("Input provided is 0 bytes long")
}

pub fn part2_compare_iter(input: &[char]) -> usize {
    let mut current_start_idx = 0;
    loop {
        let view = &input[current_start_idx..];
        if let Some(Some(skip_to_idx)) = view
            .windows(14)
            .map(|window| {
                window
                    .iter()
                    .enumerate()
                    .filter_map(|(forward_idx, c)| {
                        window[forward_idx + 1..]
                            .iter()
                            .filter_map(|cc| {
                                if c == cc {
                                    Some(current_start_idx + forward_idx)
                                } else {
                                    None
                                }
                            })
                            .next()
                    })
                    .next()
            })
            .next()
        {
            current_start_idx = skip_to_idx + 1;
        } else {
            return current_start_idx + 14;
        }
    }
}

pub fn part2_lib(input: &[char]) -> usize {
    let handheld = communication_system::Device::from(input);
    handheld.find_start_of_message()
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

    test!(part1, 7);
    test!(part1_skipping, 7);
    test!(part1_hashset, 7);
    test!(part1_skipmap, 7);
    test!(part1_compare, 7);
    test!(part1_lib, 7);
    test!(part2, 19);
    test!(part2_skipping, 19);
    test!(part2_hashset, 19);
    test!(part2_skipmap, 19);
    test!(part2_compare, 19);
    test!(part2_compare_iter, 19);
    test!(part2_lib, 19);
}
