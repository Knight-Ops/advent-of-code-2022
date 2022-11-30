pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .into_iter()
        .map(|num| num.trim().parse::<u32>().expect("Error parsing &str into u32"))
        .collect()
}

pub fn part1(input: &[u32]) -> usize {

}

pub fn part2(input: &[u32]) -> usize {

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

    test!(part1, 0);
    test!(part2, 0);
}