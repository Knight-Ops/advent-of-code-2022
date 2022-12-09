use fnv::FnvHashSet;

#[derive(Debug)]
pub enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    RightUp(i32, i32),
    RightDown(i32, i32),
    LeftUp(i32, i32),
    LeftDown(i32, i32)
}

pub struct Rope {
    knots: Vec<(i32, i32)>,
    tail_history: FnvHashSet<(i32, i32)>,
}

impl Rope {
    pub fn new(knot_length: usize) -> Self {
        let mut tail_history = FnvHashSet::default();
        tail_history.insert((0,0));
        Self {
            knots: vec![(0,0); knot_length],
            tail_history,
        }
    }

    pub fn apply_movement(&mut self, direction: &Direction) {
        match direction {
            Direction::Up(distance) => {
                for _ in 0..*distance {
                    self.knots[0].1 += 1;

                    if self.tail_move_required(1) {
                        self.move_tail(1);
                    }
                }
            }
            Direction::Down(distance) => {
                for _ in 0..*distance {
                    self.knots[0].1 -= 1;

                    if self.tail_move_required(1) {
                        self.move_tail(1);
                    }
                }
            }
            Direction::Left(distance) => {
                for _ in 0..*distance {
                    self.knots[0].0 -= 1;

                    if self.tail_move_required(1) {
                        self.move_tail(1);
                    }
                }
            }
            Direction::Right(distance) => {
                for _ in 0..*distance {
                    self.knots[0].0 += 1;

                    if self.tail_move_required(1) {
                        self.move_tail(1);
                    }
                }
            }
            _ => unimplemented!("Head can only move in cardinal direction")
        }
    }

    // Quick check to see if tail is already adjacent to us
    fn tail_move_required(&self, tail_idx: usize) -> bool {
        (self.knots[tail_idx - 1].0 - self.knots[tail_idx].0).abs() > 1 || (self.knots[tail_idx - 1].1 - self.knots[tail_idx].1).abs() > 1
    }

    fn move_tail(&mut self, tail_idx: usize) {
        let mut x = self.knots[tail_idx - 1].0 - self.knots[tail_idx].0;
        let mut y = self.knots[tail_idx - 1].1 - self.knots[tail_idx].1;

        if x.abs() == 2 {
            x = x >> 1;
        } 
        if y.abs() == 2 {
            y = y >> 1;
        }

        self.knots[tail_idx].0 += x;
        self.knots[tail_idx].1 += y;
        
        if tail_idx == self.knots.len() - 1 {
            self.tail_history.insert(*self.knots.last().unwrap());
        } else {
            if self.tail_move_required(tail_idx + 1) {
                // Tailcall optimzation should make this okay-ish
                self.move_tail(tail_idx + 1)
            }
        }
    }

    pub fn get_unique_tail_locations(&self) -> usize {
        self.tail_history.len()
    }
}


pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            match split
                .next()
                .expect("Could not retrieve direction from input line")
            {
                "U" => Direction::Up(
                    split
                        .next()
                        .expect("Could not retrieve value from input line")
                        .parse()
                        .expect("Failed to parse distance from input line"),
                ),
                "D" => Direction::Down(
                    split
                        .next()
                        .expect("Could not retrieve value from input line")
                        .parse()
                        .expect("Failed to parse distance from input line"),
                ),
                "L" => Direction::Left(
                    split
                        .next()
                        .expect("Could not retrieve value from input line")
                        .parse()
                        .expect("Failed to parse distance from input line"),
                ),
                "R" => Direction::Right(
                    split
                        .next()
                        .expect("Could not retrieve value from input line")
                        .parse()
                        .expect("Failed to parse distance from input line"),
                ),
                _ => unimplemented!("This direction shouldn't exist"),
            }
        })
        .collect()
}

pub fn part1(input: &[Direction]) -> usize {
    let mut rope = Rope::new(2);

    input.iter().for_each(|dir| rope.apply_movement(dir));

    rope.get_unique_tail_locations()
}

pub fn part2(input: &[Direction]) -> usize {
    let mut rope = Rope::new(10);

    input.iter().for_each(|dir| {
        rope.apply_movement(dir);
    });

    rope.get_unique_tail_locations()
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

    test!(part1, 13);
    test!(part2, 1);
}
