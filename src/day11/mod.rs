use std::collections::VecDeque;
use std::cell::{Cell};
use std::fmt::{Debug};

#[derive(Debug)]
pub struct BarrelOfMonkeys {
    barrel: Vec<Monkey>,
    barrel_modulo: usize,
}

impl BarrelOfMonkeys {
    pub fn process_round(&mut self) {
        self.barrel.iter().for_each(|monkey| {
            // We can just straight up take the VeqDeque since we know when we are done, it will be empty, this allows u to avoid mutability of the whole `Monkey`
            let mut test_vecdeq = monkey.items.replace(VecDeque::new());
            while let Some(item) = test_vecdeq.pop_front() {
                let mut item = monkey.inspect_item(item);

                item = item / 3;

                if monkey.test_item(item) {
                    let mut true_monkey_items = self.barrel[monkey.monkey_true].items.take();

                    true_monkey_items.push_back(item);

                    self.barrel[monkey.monkey_true].items.set(true_monkey_items);
                } else {
                    let mut false_monkey_items = self.barrel[monkey.monkey_false].items.take();

                    false_monkey_items.push_back(item);

                    self.barrel[monkey.monkey_true].items.set(false_monkey_items);
                }

            }
        })
    }

    pub fn process_round_crazy(&mut self) {
        self.barrel.iter().for_each(|monkey| {
            // We can just straight up take the VeqDeque since we know when we are done, it will be empty, this allows u to avoid mutability of the whole `Monkey`
            let mut test_vecdeq = monkey.items.replace(VecDeque::new());

            while let Some(item) = test_vecdeq.pop_front() {
                let mut item = monkey.inspect_item(item);

                item = item % self.barrel_modulo;

                if monkey.test_item(item) {
                    let mut true_monkey_items = self.barrel[monkey.monkey_true].items.take();

                    true_monkey_items.push_back(item);

                    self.barrel[monkey.monkey_true].items.set(true_monkey_items);
                } else {
                    let mut false_monkey_items = self.barrel[monkey.monkey_false].items.take();

                    false_monkey_items.push_back(item);

                    self.barrel[monkey.monkey_true].items.set(false_monkey_items);
                }

            }
        })
    }

    pub fn process_round_crazy_unsafe(&mut self) {
        self.barrel.iter().for_each(|monkey| {
            // We can just straight up take the VeqDeque since we know when we are done, it will be empty, this allows u to avoid mutability of the whole `Monkey`
            let mut test_vecdeq = monkey.items.replace(VecDeque::new());

            while let Some(item) = test_vecdeq.pop_front() {
                let mut item = monkey.inspect_item(item);

                item = item % self.barrel_modulo;

                if monkey.test_item(item) {
                    unsafe {      
                        // let mut true_monkey_items = self.barrel[monkey.monkey_true].items.take();
                        let mut true_monkey_items = self.barrel.get_unchecked(monkey.monkey_true).items.take();

                        true_monkey_items.push_back(item);
    
                        // self.barrel[monkey.monkey_true].items.set(true_monkey_items);
                        self.barrel.get_unchecked(monkey.monkey_true).items.set(true_monkey_items);
                    }

                } else {
                    unsafe {
                        // let mut false_monkey_items = self.barrel[monkey.monkey_false].items.take();
                        let mut false_monkey_items = self.barrel.get_unchecked(monkey.monkey_false).items.take();
    
                        false_monkey_items.push_back(item);
    
                        // self.barrel[monkey.monkey_true].items.set(false_monkey_items);
                        self.barrel.get_unchecked(monkey.monkey_false).items.set(false_monkey_items);

                    }
                }

            }
        })
    }
}

pub struct Monkey {
    // By using a Cell instead of a RefCell here, we can avoid the runtime borrow checking which is lots of overhead. Since this is single threaded, not big deal.
    items: Cell<VecDeque<usize>>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    mod_value: usize,
    monkey_true: usize,
    monkey_false: usize,
    inspection_count: Cell<usize>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Inspected : {}", self.inspection_count.get())
    }
}

impl Monkey {
    pub fn inspect_item(&self, item: usize) -> usize {
        self.inspection_count.update(|x| x + 1);

        (self.operation)(item)
    }

    pub fn test_item(&self, item: usize) -> bool {
        (self.test)(item)
    }
}

pub fn input_generator(input: &str) -> BarrelOfMonkeys {
    let barrel: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey_block| {
            let mut lines = monkey_block.lines();

            // This is "Monkey X:" we don't care.
            lines.next();

            // "Starting items: X"
            let items = lines.next().unwrap().split(": ").nth(1).unwrap().split(", ").map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<usize>>();

            // "Operation: X" We are going to just ignore the left side of the equal sign, but this could be an invalid assumption at some point
            let mut operation_builder = lines.next().unwrap().split(" = ").nth(1).unwrap().split_whitespace();
            if operation_builder.next().unwrap() != "old" {
                unimplemented!("Support only includes old being in the first parameter")
            }
            let op_sign = operation_builder.next().unwrap();
            // If we get Some(usize) then it parsed successfully, if we see old, we will just get a None
            let rhs = operation_builder.next().unwrap().parse::<usize>().ok();

            let operation: Box<dyn Fn(usize) -> usize> = match (op_sign, rhs) {
                ("+", Some(val))=> Box::new(move |x| x + val),
                ("+", None) => Box::new(|x| x + x),
                ("*", Some(val)) => Box::new(move |x| x.wrapping_mul(val)),
                ("*", None) => Box::new(|x| x.wrapping_mul(x)),
                _ => unimplemented!("This operation is not supported")
            };

            // We have no real information about this spec, so we are going to assume all tests are "Divisible by X"
            let test_val = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();
            let test = move |x| x % test_val == 0;

            // All this line is fluff except the last number
            let monkey_true = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

            // All this line is fluff except the last number
            let monkey_false = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

            Monkey { items: Cell::new(items), operation: Box::new(operation), test: Box::new(test), mod_value: test_val, monkey_true, monkey_false, inspection_count: Cell::new(0) }
        })
        .collect();

        let modulo = barrel.iter().map(|monkey| monkey.mod_value).product();

        BarrelOfMonkeys { barrel, barrel_modulo: modulo }
}

pub fn part1(input: &mut BarrelOfMonkeys) -> usize {

    for _ in 0..20 {
        input.process_round();
    }

    let mut max = 0;
    let mut second_max = 0;
    for monkey in input.barrel.iter() {
        if monkey.inspection_count.get() > max {
            if max > second_max {
                second_max = max;
            }
            max = monkey.inspection_count.get();
        } else if monkey.inspection_count.get() > second_max {
            second_max = monkey.inspection_count.get();
        }
    }

    max * second_max
}

pub fn part2(input: &mut BarrelOfMonkeys) -> usize {
    for _ in 0..10000 {
        input.process_round_crazy();
    }

    let mut max = 0;
    let mut second_max = 0;
    for monkey in input.barrel.iter() {
        if monkey.inspection_count.get() > max {
            if max > second_max {
                second_max = max;
            }
            max = monkey.inspection_count.get();
        } else if monkey.inspection_count.get() > second_max {
            second_max = monkey.inspection_count.get();
        }
    }

    max * second_max
}

pub fn part2_unsafe(input: &mut BarrelOfMonkeys) -> usize {
    for _ in 0..10000 {
        input.process_round_crazy_unsafe();
    }

    let mut max = 0;
    let mut second_max = 0;
    for monkey in input.barrel.iter() {
        if monkey.inspection_count.get() > max {
            if max > second_max {
                second_max = max;
            }
            max = monkey.inspection_count.get();
        } else if monkey.inspection_count.get() > second_max {
            second_max = monkey.inspection_count.get();
        }
    }

    max * second_max
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

    test_mut!(part1, 10605);
    test_mut!(part2, 2713310158);
    test_mut!(part2_unsafe, 2713310158);
}