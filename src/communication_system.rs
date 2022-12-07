use std::convert::From;

pub struct Device {
    data_stream: Vec<char>,
}

impl Device {
    pub fn new(data_stream: &str) -> Self {
        Self {
            data_stream: data_stream.chars().collect(),
        }
    }

    // This implementation was the winner from day 6 testing
    fn find_duplicate_data(&self, size: usize) -> usize {
        let mut start_idx = 0;

        'outer: while start_idx < self.data_stream.len() {
            for (idx, c) in self.data_stream[start_idx..start_idx + size]
                .iter()
                .enumerate()
            {
                for cc in self.data_stream[start_idx + idx + 1..start_idx + size].iter() {
                    if c == cc {
                        start_idx += idx + 1;
                        continue 'outer;
                    }
                }
            }
            return start_idx + size;
        }

        unreachable!("Input provided is 0 bytes long")
    }

    pub fn get_data(&self, start_offset: usize) -> &[char] {
        &self.data_stream[start_offset..]
    }

    pub fn find_start_of_packet(&self) -> usize {
        self.find_duplicate_data(4)
    }

    pub fn find_start_of_message(&self) -> usize {
        self.find_duplicate_data(14)
    }
}

impl From<&[char]> for Device {
    fn from(value: &[char]) -> Self {
        Self {
            data_stream: value.to_owned().into_iter().collect(),
        }
    }
}
