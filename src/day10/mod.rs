#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Addx(isize),
    Noop
}

#[derive(Debug)]
pub struct CPU<'a> {
    x: isize,
    cycle: usize,
    execution_cycle: usize,
    program_counter: usize,
    state: CpuState,
    instructions: &'a [Instruction]
}

impl<'a> CPU<'a> {
    pub fn new(program: &'a [Instruction]) -> Self {
        Self {
            x: 1,
            cycle: 1,
            execution_cycle: 0,
            program_counter: 0,
            instructions: program,
            state: CpuState::Execute,
        }
    }

    pub fn execute_cycle(&mut self) {
        match self.state {
            CpuState::Fetch => {
                // We use Fetch state to basically clear instruction counters, but it doesn't actually take a cycle, so just call ourselves again
                self.execution_cycle = 0;
                self.program_counter += 1;

                if self.program_counter >= self.instructions.len() {
                    return;
                }

                self.state = CpuState::Execute;
                return self.execute_cycle();
            },
            CpuState::Execute => {
                match self.instructions[self.program_counter] {
                    Instruction::Noop => {
                        // We can skip incrementing the execution cycle counter, because its a single cycle instruction
                        self.state = CpuState::Fetch;
                    }
                    Instruction::Addx(val) => {
                        match self.execution_cycle {
                            0 => {},
                            1 => {
                                self.state = CpuState::Fetch;
                                self.x += val;
                            }
                            _ => panic!("Addx is only a two cycle instruction")
                        }
                        self.execution_cycle += 1;
                    }
                }

                self.cycle += 1;
            }
        }
    }
}

#[derive(Debug)]
pub enum CpuState {
    Fetch,
    Execute
}

pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            match split.next().unwrap() {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(split.next().unwrap().parse::<isize>().unwrap()),
                _ => unimplemented!("This instruction is unimplemented")
            }
        })       
        .collect()
}

pub fn part1(input: &[Instruction]) -> isize {
    let mut accumulator = 0;
    let mut cpu = CPU::new(input);

    loop {
        cpu.execute_cycle();

        if cpu.cycle == 20 || cpu.cycle == 60 || cpu.cycle == 100 || cpu.cycle == 140 || cpu.cycle == 180 || cpu.cycle == 220 {
            accumulator += cpu.cycle as isize * cpu.x
        }

        if cpu.cycle > 220 {
            break;
        }
    }
   
    accumulator
}

pub fn part2(input: &[Instruction]) -> usize {
    let mut cpu = CPU::new(input);

    // We don't need this much space, but its page aligned which we hope makes the allocator happy
    let mut display = String::with_capacity(4096);
    loop {
        if (cpu.x-1..=cpu.x+1).contains(&(((cpu.cycle-1) % 40) as isize)) {
            display.push('#')
        } else {
            display.push(' ')
        }

        cpu.execute_cycle();
        
        if cpu.cycle % 40 == 1 {
            display.push('\n')
        }

        if cpu.cycle > 240 {
            break;
        }
    }

    // println!("{}", display);
    0
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

    test!(part1, 13140);
    test!(part2, 0);
}