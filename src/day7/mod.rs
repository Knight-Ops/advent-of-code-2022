use fnv::FnvHashMap;

#[derive(Debug)]
pub struct HandheldNavigator {
    current_directory: String,
    file_system: FnvHashMap<String, HandheldDirectory>,
}

impl HandheldNavigator {
    pub fn new() -> Self {
        let mut fs = FnvHashMap::default();
        fs.insert(
            "/".to_string(),
            HandheldDirectory {
                parent: None,
                sub_directories: Vec::new(),
                file_list: Vec::new(),
                total_size: 0,
            },
        );

        Self {
            current_directory: "/".to_string(),
            file_system: fs,
        }
    }

    pub fn apply_output(&mut self, output: &HandheldUpdateOutput) {
        match output {
            HandheldUpdateOutput::Command(cmd) => {
                // It may be wise to build a state machine to add error checking here where an "ls" just changes state to injesting output
                match cmd {
                    HandheldCommandType::ChangeDirectory(arg) => match arg.as_str() {
                        "/" => {
                            // We can unwrap here because we know this exists
                            self.current_directory = "/".to_string();
                        }
                        ".." => {
                            if let Some(current_dir) = self.file_system.get(&self.current_directory)
                            {
                                if let Some(parent_dir) = current_dir.parent.to_owned() {
                                    self.current_directory = parent_dir.to_string()
                                } else {
                                    panic!("Directory does not have a parent")
                                }
                            } else {
                                panic!("Directory does not exist in current structure")
                            }
                        }
                        _ => {
                            let full_path = format!("{}{}/", self.current_directory, arg);
                            if let Some(_) = self.file_system.get(&full_path) {
                                self.current_directory = full_path
                                    
                            } else {
                                panic!("Tried to cd into directory that doesn't exist")
                            }
                        }
                    },
                    HandheldCommandType::ListDirectory => {
                        // There isn't anything to do here
                    }
                }
            }
            HandheldUpdateOutput::Directory(name) => {
                let path_name = format!("{}{}/", self.current_directory, name);
                self.file_system.insert(
                    path_name,
                    HandheldDirectory {
                        parent: Some(self.current_directory.to_owned()),
                        sub_directories: Vec::new(),
                        file_list: Vec::new(),
                        total_size: 0,
                    },
                );

                self.file_system
                    .get_mut(&self.current_directory)
                    .unwrap()
                    .sub_directories
                    .push(name.to_owned())
            }
            HandheldUpdateOutput::File(name, size) => {
                if let Some(mut parent) = {
                    let mut current_dir =
                        self.file_system.get_mut(&self.current_directory).unwrap();
                    current_dir
                        .file_list
                        .push(HandheldFile { name: name.to_owned(), size: *size });
                    current_dir.total_size += *size;
                    current_dir.parent.to_owned()
                } {
                    // Add this back up to the top
                    loop {
                        let current_dir = self.file_system.get_mut(&parent).unwrap();
                        current_dir.total_size += *size;

                        if current_dir.parent.is_none() {
                            break;
                        } else {
                            parent = current_dir.parent.to_owned().unwrap()
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum HandheldUpdateOutput {
    Command(HandheldCommandType),
    Directory(String),
    File(String, usize),
}

#[derive(Debug)]
pub enum HandheldCommandType {
    ChangeDirectory(String),
    ListDirectory,
}

#[derive(Debug)]
pub struct HandheldCommand {
    command: HandheldCommandType,
}

#[derive(Debug)]
pub struct HandheldDirectory {
    parent: Option<String>,
    sub_directories: Vec<String>,
    file_list: Vec<HandheldFile>,
    total_size: usize,
}

#[derive(Debug)]
pub struct HandheldFile {
    name: String,
    size: usize,
}

pub fn input_generator(input: &str) -> HandheldNavigator {
    let mut handheld = HandheldNavigator::new();

    
    let output_lines : Vec<HandheldUpdateOutput> = input
        .lines()
        .map(|line| {
            if line.starts_with('$') {
                let mut split = line.split_whitespace();

                // Discard the $ we already matched on
                split.next();

                let command = match split
                    .next()
                    .expect("Command line doesn't contain data for an actual command")
                {
                    "cd" => {
                        let argument = split
                            .next()
                            .expect("Change directory command found, but no argument provided");
                        HandheldCommandType::ChangeDirectory(argument.to_owned())
                    }
                    "ls" => HandheldCommandType::ListDirectory,
                    _ => unimplemented!("This command does not exist"),
                };

                HandheldUpdateOutput::Command(command)
            } else if line.starts_with("dir") {
                let mut split = line.split_whitespace();

                // Discard our already matched "dir"
                split.next();

                let dir_name = split.next().expect("dir output found without name");

                HandheldUpdateOutput::Directory(dir_name.to_owned())
            } else {
                // This is a hack for files, I don't like it being the fallback
                let mut split = line.split_whitespace();

                let file_size = split
                    .next()
                    .expect("File being parsed without size")
                    .parse()
                    .expect("File size failed to parse into a number");
                let file_name = split.next().expect("File being parsed without name");
                HandheldUpdateOutput::File(file_name.to_owned(), file_size)
            }
        })
        .collect();

        output_lines.iter().for_each(|output| {
            handheld.apply_output(output);
        });

        handheld
}

pub fn part1(input: &HandheldNavigator) -> usize {
    input
        .file_system
        .iter()
        .filter_map(|(_, dir)| {
            if dir.total_size < 100000 {
                Some(dir.total_size)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &HandheldNavigator) -> usize {
    let free_space = 70000000 - input.file_system.get(&"/".to_string()).unwrap().total_size;
    let size_needed = 30000000 - free_space;

    input.file_system.iter().filter_map(|(_, dir)| {
        if dir.total_size > size_needed {
            Some(dir.total_size)
        } else {
            None
        }
    }).min().unwrap()
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

    test!(part1, 95437);
    test!(part2, 24933642);
}
