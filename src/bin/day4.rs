const _TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
const INPUT: &str = include_str!("day4.in");

struct Card {
    winning: &'static str,
    potentials: &'static str,
}

fn main() {
    let input = INPUT;
    let cards: Box<[Card]> = input
        .lines()
        .map(|line| {
            let colon = line.find(':').unwrap();
            let bar = line.find('|').unwrap();

            Card {
                winning: line[colon + 1..bar].trim(),
                potentials: line[bar + 1..].trim(),
            }
        })
        .collect();

    println!("[PART 1 ANSWER] {}", part_one_solution(&cards));
    println!("[PART 2 ANSWER] {}", part_two_solution(&cards));
}

fn part_one_solution(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| {
            let winning: Box<[&str]> = card.winning.split_whitespace().collect();

            let points = card
                .potentials
                .split_whitespace()
                .map(|num| winning.contains(&num) as u32)
                .sum::<u32>();

            if let Some(points) = points.checked_sub(1) {
                2_u32.pow(points)
            } else {
                0
            }
        })
        .sum()
}

fn part_two_solution(cards: &[Card]) -> u32 {
    let win_iter = cards.iter().map(|card| {
        let winning: Box<[&str]> = card.winning.split_whitespace().collect();

        let points = card
            .potentials
            .split_whitespace()
            .map(|num| winning.contains(&num) as u32)
            .sum::<u32>();

        points
    });
    let mut copies: Box<[u32]> = vec![1; win_iter.len()].into();

    for (idx, card) in win_iter.enumerate() {
        let curr_copy = copies[idx];

        for copy in copies[idx + 1..idx + 1 + card as usize].iter_mut() {
            *copy += curr_copy;
        }
    }

    copies.iter().sum()
}
