use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug)]
pub struct TreeFarm {
    row_length: usize,
    trees: Vec<u8>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    RowRight(usize),
    RowLeft(usize),
    ColumnUp(usize),
    ColumnDown(usize),
}


pub fn input_generator(input: &str) -> TreeFarm {
    let mut trees = Vec::new();
    let mut row_length = 0;

    input
        .lines()
        .for_each(|line| {
            if row_length == 0 {
                row_length = line.trim().len();
            }
            
            line.chars().for_each(|c| {
                if c as u8 > 0x39 ||( c as u8) < 0x30 {
                    panic!("Hacky conversion is not for you.")
                }
                trees.push(c as u8 - b'0');
            })
        });

    if row_length * row_length != trees.len() {
        panic!("Assumed we had a square tree farm");
    }

    TreeFarm { row_length, trees }
}

pub fn part1(input: &TreeFarm) -> usize {
    input.trees.iter().enumerate().filter(|(idx, tree_height)| {
        if idx < &input.row_length {
            // First Row
            true
        } else if idx % input.row_length == 0 {
            // First Column
            true
        } else if idx % input.row_length == input.row_length - 1 {
            // Last Column
            true
        } else if idx > &(input.row_length * (input.row_length - 1)) {
            // Last Row
            true
        } else {
            // Check Left
            let start_of_row = (idx / input.row_length) * input.row_length;
            let mut distance = 1;
            loop {
                if &&input.trees[idx - distance] >= tree_height {
                    break
                }

                if idx - distance == start_of_row {
                    return true
                }

                distance += 1;
            }

            // Check Right
            let end_of_row = (((idx / input.row_length) + 1) * input.row_length) - 1;
            distance = 1;
            loop {
                if &&input.trees[idx + distance] >= tree_height {
                    break
                }

                if idx + distance == end_of_row {
                    return true
                }

                distance += 1;
            }

            // Check Up
            let start_of_column = idx % input.row_length;
            distance = 1;
            loop {
                if &&input.trees[idx - distance * input.row_length] >= tree_height {
                    break
                }

                if idx - distance * input.row_length == start_of_column {
                    return true
                }

                distance += 1;
            }

            // Check Down
            let end_of_column = idx % input.row_length + (input.row_length * (input.row_length - 1));
            distance = 1;
            loop {
                if &&input.trees[idx + distance * input.row_length] >= tree_height {
                    break
                }

                if idx + distance * input.row_length == end_of_column {
                    return true
                }

                distance += 1;
            }

            false
        }
    }).count()
}

pub fn part1_confusing(input: &TreeFarm) -> usize {
    let mut set = FnvHashSet::default();

    (0..input.row_length).for_each(|x| {
        set.insert((x, 0));
        set.insert((0, x));
        set.insert((input.row_length - 1, x));
        set.insert((x, input.row_length - 1));
    });

    // We can skip the first and last of each row/column because all outside trees are visible, no point in checking
    for x in 1..input.row_length-1 {
        // Rows from left to right
        let mut max_height = input.trees[input.row_length*x];
        for y in 1..input.row_length {
            if input.trees[input.row_length*x+y] > max_height{
                max_height = input.trees[input.row_length*x+y];
                set.insert((x, y));
            }

            if max_height == 9 {
                // Early exit if we have a max height tree
                break
            }
        }
    
        // Rows right to left
        max_height = input.trees[input.row_length*x + input.row_length-1];
        for y in 1..input.row_length {
            if input.trees[input.row_length*x+input.row_length-1 - y] > max_height{
                max_height = input.trees[input.row_length*x+input.row_length-1 - y];
                set.insert((x, y));
            }

            if max_height == 9 {
                // Early exit if we have a max height tree
                break
            }
        }

        // Column Top to Bottom
        max_height = input.trees[x];
        for y in 1..input.row_length {
            if input.trees[x+input.row_length*y] > max_height{
                max_height = input.trees[x+input.row_length*y];
                set.insert((x, y));
            }

            if max_height == 9 {
                // Early exit if we have a max height tree
                break
            }
        }

        // Column Top to Bottom
        max_height = input.trees[input.row_length * (input.row_length - 1) + x];
        for y in 1..input.row_length {
            if input.trees[input.row_length * (input.row_length - 1) + x - y*input.row_length] > max_height{
                max_height = input.trees[input.row_length * (input.row_length - 1) + x - y*input.row_length];
                set.insert((x, y));
            }

            if max_height == 9 {
                // Early exit if we have a max height tree
                break
            }
        }
    }

    set.len()
}

pub fn part2(input: &TreeFarm) -> usize {
    input.trees.iter().enumerate().map(|(idx, tree_height)| {
        if idx < input.row_length {
            // First Row
            0
        } else if idx % input.row_length == 0 {
            // First Column
            0
        } else if idx % input.row_length == input.row_length - 1 {
            // Last Column
            0
        } else if idx > input.row_length * (input.row_length - 1) {
            // Last Row
            0
        } else {
            // Check Left
            let start_of_row = (idx / input.row_length) * input.row_length;
            let mut distance = 1;
            let mut left = 0;
            loop {
                if &input.trees[idx - distance] >= tree_height {
                    left += 1;
                    break
                }

                if idx - distance == start_of_row {
                    left += 1;
                    break
                }

                distance += 1;
                left += 1;
            }

            // Check Right
            let end_of_row = (((idx / input.row_length) + 1) * input.row_length) - 1;
            distance = 1;
            let mut right = 0;
            loop {
                if &input.trees[idx + distance] >= tree_height {
                    right += 1;
                    break
                }

                if idx + distance == end_of_row {
                    right += 1;
                    break
                }

                distance += 1;
                right += 1;
            }

            // Check Up
            let start_of_column = idx % input.row_length;
            distance = 1;
            let mut up = 0;
            loop {
                if &input.trees[idx - distance * input.row_length] >= tree_height {
                    up += 1;
                    break
                }

                if idx - distance * input.row_length == start_of_column {
                    up += 1;
                    break
                }

                distance += 1;
                up += 1;
            }

            // Check Down
            let end_of_column = idx % input.row_length + (input.row_length * (input.row_length - 1));
            distance = 1;
            let mut down = 0;
            loop {
                if &input.trees[idx + distance * input.row_length] >= tree_height {
                    down += 1;
                    break
                }

                if idx + distance * input.row_length == end_of_column {
                    down += 1;
                    break
                }

                distance += 1;
                down += 1;
            }

            left * right * up * down
        }
    }).max().unwrap()
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

    test!(part1, 21);
    test!(part1_confusing, 21);
    test!(part2, 8);
}