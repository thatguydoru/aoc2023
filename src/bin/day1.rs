const INPUT: &str = include_str!("day1.in");

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut sum: u32 = 0;

    for line in INPUT.lines() {
        let digits: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let digits = digits.as_bytes();

        sum += (digits[0] - 48) as u32 * 10;
        sum += (digits[digits.len() - 1] - 48) as u32;
    }

    println!("[ANSWER]\t{sum}");
}

fn part_two() {
    let mut sum = 0;

    for line in INPUT.lines() {
        let mut digits = Vec::new();

        for start in 0..line.len() {
            for end in start + 1..=line.len() {
                let digit = match &line[start..end] {
                    "1" | "one" => Some(1),
                    "2" | "two" => Some(2),
                    "3" | "three" => Some(3),
                    "4" | "four" => Some(4),
                    "5" | "five" => Some(5),
                    "6" | "six" => Some(6),
                    "7" | "seven" => Some(7),
                    "8" | "eight" => Some(8),
                    "9" | "nine" => Some(9),
                    _ => None,
                };

                if let Some(digit) = digit {
                    digits.push(digit);
                    break;
                }
            }
        }

        sum += digits[0] * 10;
        sum += digits[digits.len() - 1];
    }

    println!("[ANSWER]\t{sum}");
}
