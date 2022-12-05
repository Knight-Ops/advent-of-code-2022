use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct CleaningAssignment {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

pub fn input_generator(input: &str) -> Vec<CleaningAssignment> {
    input
        .lines()
        .map(|line| {
            let mut ranges = line.split(",").map(|range| {
                let mut range_iter = range
                    .split("-")
                    .map(|num| num.parse::<u32>().expect("Error while parsing str to u32"));

                range_iter.next().expect("Error while parsing input")
                    ..=range_iter.next().expect("Error while parsing input")
            });

            CleaningAssignment {
                first: ranges.next().expect("Error while retrieving range entry"),
                second: ranges.next().expect("Error while retrieving range entry"),
            }
        })
        .collect()
}

pub fn part1(input: &[CleaningAssignment]) -> usize {
    input
        .iter()
        .filter_map(|cleaning_assignment| {
            // This is ugly logic that should get most of the checks out of the way all at once
            match (
                cleaning_assignment.first.start() >= cleaning_assignment.second.start(),
                cleaning_assignment.first.end() <= cleaning_assignment.second.end(),
            ) {
                // This is the simple case that the first range is totally included in the second range
                (true, true) => Some(true),
                // This is the simple case that the second range is totally included in the first range
                (false, false) => Some(true),
                // Logic gets trickier here, this is where both ranges *could* start at the same location
                (true, false) => {
                    // If the first and second range start at the same spot, this means the second range is included in the first
                    if cleaning_assignment.first.start() == cleaning_assignment.second.start() {
                        Some(true)
                    } else {
                        None
                    }
                }
                (false, true) => {
                    // If the first and second range end at the same spot, this means that first range is included in the second
                    if cleaning_assignment.first.end() == cleaning_assignment.second.end() {
                        Some(true)
                    } else {
                        None
                    }
                }
            }
        })
        .count()
}

pub fn part2(input: &[CleaningAssignment]) -> usize {
    input
        .iter()
        .filter_map(|cleaning_assignment| {
            match (
                cleaning_assignment.first.end() < cleaning_assignment.second.start(),
                cleaning_assignment.first.start() > cleaning_assignment.second.end(),
            ) {
                (true, true) => panic!("Something horrible has happened, this is impossible"),
                (true, false) => None,
                (false, true) => None,
                _ => Some(true),
            }
        })
        .count()
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

    test!(part1, 2);
    test!(part2, 4);
}
