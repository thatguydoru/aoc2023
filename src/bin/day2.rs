use std::cell::OnceCell;

const INPUT: &str = include_str!("day2.in");

thread_local! {
    static GAMES: OnceCell<Vec<Game>> = OnceCell::new();
}

#[derive(Debug, PartialEq)]
enum Cubes {
    Red(i32),
    Blue(i32),
    Green(i32),
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    subsets: Vec<Vec<Cubes>>,
}

#[derive(Debug, Default)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    GAMES.with(|games| {
        games
            .set(parse_input(INPUT))
            .expect("[ERROR] failed to initialize games")
    });

    part_one::solution();
    part_two::solution();
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|lines| {
            let mut split = lines.split(':');
            let id = split
                .next()
                .and_then(|game| {
                    game.split_whitespace()
                        .last()
                        .and_then(|id| id.parse::<u32>().ok())
                })
                .unwrap();

            let subsets: Vec<Vec<Cubes>> = split
                .next()
                .map(|subsets| subsets.split(';').map(parse_subset).collect())
                .unwrap();

            Game { id, subsets }
        })
        .collect()
}

fn parse_subset(subset: &str) -> Vec<Cubes> {
    subset
        .split(',')
        .map(|cubes| {
            let mut cubes = cubes.split_whitespace();
            let count: i32 = cubes.next().and_then(|count| count.parse().ok()).unwrap();
            let color = cubes.next().unwrap();

            match color {
                "red" => Cubes::Red(count),
                "blue" => Cubes::Blue(count),
                "green" => Cubes::Green(count),
                _ => unreachable!("[ERROR] unknown color"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = vec![
            Game {
                id: 1,
                subsets: vec![
                    vec![Cubes::Blue(3), Cubes::Red(4)],
                    vec![Cubes::Red(1), Cubes::Green(2), Cubes::Blue(6)],
                    vec![Cubes::Green(2)],
                ],
            },
            Game {
                id: 2,
                subsets: vec![
                    vec![Cubes::Blue(1), Cubes::Green(2)],
                    vec![Cubes::Green(3), Cubes::Blue(4), Cubes::Red(1)],
                    vec![Cubes::Green(1), Cubes::Blue(1)],
                ],
            },
        ];

        assert_eq!(parse_input(input), expected);
    }
}

mod part_one {
    use super::*;

    const BAG: Bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    pub fn solution() {
        GAMES.with(|games| {
            let sum: u32 = part_one::filter_possible_games(games.get().unwrap(), &BAG).sum();

            println!("[PART 1 RESULT] {sum}");
        });
    }

    fn filter_possible_games<'a>(
        games: &'a [Game],
        bag: &'a Bag,
    ) -> impl Iterator<Item = u32> + 'a {
        games.iter().filter_map(|g| {
            if part_one::is_game_possible(g, bag) {
                Some(g.id)
            } else {
                None
            }
        })
    }

    fn is_game_possible(game: &Game, bag: &Bag) -> bool {
        for subset in &game.subsets {
            let mut collected = Bag::default();
            for cubes in subset {
                match *cubes {
                    Cubes::Red(count) => collected.red = count,
                    Cubes::Green(count) => collected.green = count,
                    Cubes::Blue(count) => collected.blue = count,
                }
            }

            if bag.red < collected.red || bag.green < collected.green || bag.blue < collected.blue {
                return false;
            }
        }

        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_game_possible() {
            /*
             * Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
             * Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
             * Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
             * Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
             * Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
             * */

            let games = vec![
                Game {
                    id: 1,
                    subsets: vec![
                        vec![Cubes::Blue(3), Cubes::Red(4)],
                        vec![Cubes::Red(1), Cubes::Green(2), Cubes::Blue(6)],
                        vec![Cubes::Green(2)],
                    ],
                },
                Game {
                    id: 2,
                    subsets: vec![
                        vec![Cubes::Blue(1), Cubes::Green(2)],
                        vec![Cubes::Green(3), Cubes::Blue(4), Cubes::Red(1)],
                        vec![Cubes::Green(1), Cubes::Blue(1)],
                    ],
                },
                Game {
                    id: 3,
                    subsets: vec![
                        vec![Cubes::Green(8), Cubes::Blue(6), Cubes::Red(20)],
                        vec![Cubes::Blue(5), Cubes::Red(4), Cubes::Green(13)],
                        vec![Cubes::Green(5), Cubes::Red(1)],
                    ],
                },
                Game {
                    id: 4,
                    subsets: vec![
                        vec![Cubes::Green(1), Cubes::Red(3), Cubes::Blue(6)],
                        vec![Cubes::Green(3), Cubes::Red(6)],
                        vec![Cubes::Green(3), Cubes::Blue(15), Cubes::Red(14)],
                    ],
                },
                Game {
                    id: 5,
                    subsets: vec![
                        vec![Cubes::Red(6), Cubes::Blue(1), Cubes::Green(3)],
                        vec![Cubes::Blue(2), Cubes::Red(1), Cubes::Green(2)],
                    ],
                },
            ];

            let possible_games = vec![1, 2, 5];

            assert_eq!(
                part_one::filter_possible_games(&games, &BAG).collect::<Vec<u32>>(),
                possible_games
            );
        }
    }
}

mod part_two {
    use super::*;

    pub fn solution() {
        GAMES.with(|games| {
            let mut sum = 0;

            for game in games.get().unwrap() {
                let mut subset_iter = game.subsets.iter();
                let mut min = Bag::default();
                let first_set = subset_iter.next().unwrap();

                for cubes in first_set {
                    match *cubes {
                        Cubes::Red(count) => min.red = count,
                        Cubes::Blue(count) => min.blue = count,
                        Cubes::Green(count) => min.green = count,
                    }
                }

                for subset in subset_iter {
                    for cubes in subset {
                        match *cubes {
                            Cubes::Red(count) => {
                                if min.red < count {
                                    min.red = count;
                                }
                            }
                            Cubes::Blue(count) => {
                                if min.blue < count {
                                    min.blue = count;
                                }
                            }
                            Cubes::Green(count) => {
                                if min.green < count {
                                    min.green = count;
                                }
                            }
                        }
                    }
                }

                sum += min.red * min.blue * min.green;
            }

            println!("[PART 2 RESULT] {sum}");
        });
    }
}
