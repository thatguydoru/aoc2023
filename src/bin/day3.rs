use std::collections::HashMap;
use std::str::FromStr;

const _TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
const INPUT: &str = include_str!("day3.in");

fn main() {
    let input = INPUT;
    let mut cache: HashMap<char, Vec<Vec<u32>>> = HashMap::new();
    let line_len = input.lines().count(); // the input should be of size n x n

    for (found, symbol) in symbol_indexes(input) {
        let top = parse_line(&input[found - (line_len + 4)..=found - (line_len + 4) + 6]);
        let mid = parse_line(&input[found - 3..=found + 3]);
        let bot = parse_line(&input[found + (line_len + 4) - 6..=found + (line_len + 4)]);

        cache
            .entry(symbol)
            .and_modify(|v| v.push(top.iter().chain(&mid).chain(&bot).cloned().collect()))
            .or_insert(vec![top.iter().chain(&mid).chain(&bot).cloned().collect()]);
    }

    println!("PART 1 ANSWER: {}", part_one_solution(&cache));
    println!("PART 2 ANSWER: {}", part_two_solution(&cache));
}

fn part_one_solution(cache: &HashMap<char, Vec<Vec<u32>>>) -> u32 {
    let mut sum = 0;

    for (_, part_numbers) in cache.iter() {
        sum += part_numbers
            .iter()
            .map(|parts| parts.iter().sum::<u32>())
            .sum::<u32>();
    }

    sum
}

fn part_two_solution(cache: &HashMap<char, Vec<Vec<u32>>>) -> u32 {
    let mut sum = 0;
    let gears = cache.get(&'*').unwrap();

    for part_numbers in gears.iter().filter(|part_numbers| part_numbers.len() == 2) {
        let [a, b] = part_numbers[..] else {
            unreachable!();
        };

        sum += a * b;
    }

    sum
}

fn parse_line(line: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let ascii = line.as_bytes();
    let mid = line.len() / 2;

    if ascii[mid].is_ascii_digit() {
        let left = ascii[mid - 1].is_ascii_digit();
        let right = ascii[mid + 1].is_ascii_digit();
        let num = match (left, right) {
            (true, false) => parse_left(line, mid),
            (false, true) => parse_right(line, mid),
            (true, true) => parse_right(line, mid - 1),
            (false, false) => parse_right(line, mid),
        }
        .unwrap();

        numbers.push(num);
    } else {
        if let Ok(num) = parse_left(line, mid - 1) {
            numbers.push(num);
        }

        if let Ok(num) = parse_right(line, mid + 1) {
            numbers.push(num);
        }
    }

    numbers
}

fn symbol_indexes(s: &str) -> impl Iterator<Item = (usize, char)> + '_ {
    s.chars().enumerate().filter_map(|(idx, c)| {
        if c != '.' && !c.is_ascii_digit() && !c.is_ascii_whitespace() {
            Some((idx, c))
        } else {
            None
        }
    })
}

fn parse_left(s: &str, end: usize) -> Result<u32, <u32 as FromStr>::Err> {
    let ascii = s.as_bytes();
    let mut start = end;

    while start > 0 && ascii[start - 1].is_ascii_digit() {
        start -= 1;
    }

    s[start..=end].parse()
}

fn parse_right(s: &str, start: usize) -> Result<u32, <u32 as FromStr>::Err> {
    let ascii = s.as_bytes();
    let mut end = start;

    while end < s.len() - 1 && ascii[end + 1].is_ascii_digit() {
        end += 1;
    }

    s[start..=end].parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_left() {
        let test_input = "123....";
        let test_input1 = ".23....";
        let mid = 3;

        assert_eq!(parse_left(test_input, mid - 1), Ok(123));
        assert_eq!(parse_left(test_input1, mid - 1), Ok(23));
    }

    #[test]
    fn test_parse_right() {
        let test_input = "....123";
        let test_input1 = "....1..";
        let mid = 3;

        assert_eq!(parse_right(test_input, mid + 1), Ok(123));
        assert_eq!(parse_right(test_input1, mid + 1), Ok(1));
    }
}
