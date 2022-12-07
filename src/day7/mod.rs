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

    fn apply_text(&mut self, output: &str) {
        if output.starts_with('$') {
            let mut split = output.split_whitespace();

            // Discard the $ we already matched on
            split.next();

            match split
                .next()
                .expect("Command line doesn't contain data for an actual command")
            {
                "cd" => {
                    let argument = split
                        .next()
                        .expect("Change directory command found, but no argument provided");

                    match argument {
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
                            let full_path = format!("{}{}/", self.current_directory, argument);
                            if let Some(_) = self.file_system.get(&full_path) {
                                self.current_directory = full_path
                            } else {
                                panic!("Tried to cd into directory that doesn't exist")
                            }
                        }
                    }
                }
                "ls" => {
                    // There is nothing to do
                }
                _ => unimplemented!("This command does not exist"),
            };
        } else if output.starts_with("dir") {
            let mut split = output.split_whitespace();

            // Discard our already matched "dir"
            split.next();

            let dir_name = split.next().expect("dir output found without name");

            let path_name = format!("{}{}/", self.current_directory, dir_name);
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
                .push(dir_name.to_owned());
        } else {
            // This is a hack for files, I don't like it being the fallback
            let mut split = output.split_whitespace();

            let file_size = split
                .next()
                .expect("File being parsed without size")
                .parse()
                .expect("File size failed to parse into a number");
            let file_name = split.next().expect("File being parsed without name");

            if let Some(mut parent) = {
                let mut current_dir = self.file_system.get_mut(&self.current_directory).unwrap();
                current_dir.file_list.push(HandheldFile {
                    name: file_name.to_owned(),
                    size: file_size,
                });
                current_dir.total_size += file_size;
                current_dir.parent.to_owned()
            } {
                // Add this back up to the top
                loop {
                    let current_dir = self.file_system.get_mut(&parent).unwrap();
                    current_dir.total_size += file_size;

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

    input.lines().for_each(|line| handheld.apply_text(line));

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

    input
        .file_system
        .iter()
        .filter_map(|(_, dir)| {
            if dir.total_size > size_needed {
                Some(dir.total_size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
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
