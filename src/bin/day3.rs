use std::collections::HashMap;
use std::rc::Rc;

use regex::Regex;

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
    let cache = init_cache(input);

    println!("PART 1 ANSWER: {}", part_one_solution(&cache));
    println!("PART 2 ANSWER: {}", part_two_solution(&cache));
}

fn part_one_solution(cache: &HashMap<char, Vec<Rc<[u32]>>>) -> u32 {
    let mut sum = 0;

    for (_, part_numbers) in cache.iter() {
        sum += part_numbers
            .iter()
            .map(|parts| parts.iter().sum::<u32>())
            .sum::<u32>();
    }

    sum
}

fn part_two_solution(cache: &HashMap<char, Vec<Rc<[u32]>>>) -> u32 {
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

fn init_cache(input: &str) -> HashMap<char, Vec<Rc<[u32]>>> {
    let number_rgx = Regex::new(r"\d+").unwrap();
    let symbol_rgx = Regex::new(r"[\D--\.\n]").unwrap();
    let line_len = input.lines().count(); // input should be n x n
    let mut cache: HashMap<char, Vec<Rc<[u32]>>> = HashMap::new();

    for symbol_match in symbol_rgx.find_iter(input) {
        let symbol_idx = symbol_match.start();

        /* NOTE:
         * ....... top
         * ...*... middle
         * ....... bottom
         * The assumption here is that the number of digits on each number ranges
         * from 1 to 3. This assumption was due to the input's structure.
         * */
        let surrounding = [
            &input[symbol_idx - (line_len + 4)..=symbol_idx - (line_len + 4) + 6],
            &input[symbol_idx - 3..=symbol_idx + 3],
            &input[symbol_idx + (line_len + 4) - 6..=symbol_idx + (line_len + 4)],
        ];

        let symbol = symbol_match.as_str().chars().next().unwrap();
        let neighbors: Rc<[u32]> = surrounding
            .into_iter()
            .fold(Vec::new(), |mut accum, line| {
                accum.extend(number_rgx.find_iter(line).filter_map(|number_match| {
                    let start = number_match.start();
                    let end = number_match.end();

                    if (2..=4).contains(&start) || (2..=4).contains(&(end - 1)) {
                        Some(number_match.as_str().parse::<u32>().unwrap())
                    } else {
                        None
                    }
                }));

                accum
            })
            .into();

        cache
            .entry(symbol)
            .and_modify(|v| v.push(neighbors.clone()))
            .or_insert(vec![neighbors]);
    }

    cache
}
