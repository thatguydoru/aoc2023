const INPUT: &str = include_str!("day3.in");
#[allow(unused)]
const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn main() {
    part_one::solution();
    part_two::solution();
}

mod part_one {
    use super::*;

    pub fn solution() {
        let input: Box<[&str]> = INPUT.lines().collect();

        let inline = input
            .iter()
            .map(|line| inline_collect(line))
            .filter(|inline| !inline.is_empty());

        let compared = input
            .windows(2)
            .map(|window| {
                let [a, b] = window else {
                    unreachable!("[ERROR] len of input lines is not divisible by 2");
                };

                compare_collect(a, b)
                    .into_iter()
                    .chain(compare_collect(b, a))
                    .collect::<Vec<u32>>()
            })
            .filter(|collected| !collected.is_empty());

        let sum: Option<u32> = inline
            .chain(compared)
            .reduce(|mut a, mut b| {
                a.append(&mut b);
                a
            })
            .map(|adjacents| adjacents.into_iter().sum::<u32>());

        println!("[PART 1 RESULT] {sum:?}");
    }

    fn inline_collect(s: &str) -> Vec<u32> {
        let mut nums = Vec::new();

        for symdx in symbol_indexes(s) {
            if let Some(num) = parse_left(s, symdx) {
                nums.push(num);
            }

            if let Some(num) = parse_right(s, symdx) {
                nums.push(num);
            }
        }

        nums
    }

    fn compare_collect(a: &str, b: &str) -> Vec<u32> {
        let mut nums = Vec::new();
        let ascii = a.as_bytes();

        for symdx in symbol_indexes(b) {
            if ascii[symdx].is_ascii_digit() {
                let left = ascii.get(symdx - 1).filter(|c| c.is_ascii_digit());
                let right = ascii.get(symdx + 1).filter(|c| c.is_ascii_digit());

                let num: u32 = match (left, right) {
                    (None, Some(_)) => parse_right(a, symdx - 1).unwrap(),
                    (Some(_), None) => parse_left(a, symdx + 1).unwrap(),
                    (Some(_), Some(_)) => a[symdx - 1..=symdx + 1].parse().unwrap(),
                    (None, None) => a[symdx..=symdx].parse().unwrap(),
                };

                nums.push(num);

                continue;
            }

            if let Some(num) = parse_left(a, symdx) {
                nums.push(num);
            }

            if let Some(num) = parse_right(a, symdx) {
                nums.push(num);
            }
        }

        nums
    }

    pub fn parse_left(s: &str, start: usize) -> Option<u32> {
        let ascii = s.as_bytes();
        let mut diff = start;

        while let Some(left) = diff.checked_sub(1) {
            if !ascii[left].is_ascii_digit() {
                break;
            }
            diff = left;
        }

        s[diff..start].parse().ok()
    }

    pub fn parse_right(s: &str, start: usize) -> Option<u32> {
        let ascii = s.as_bytes();
        let mut diff = start + 1;

        while let Some(right) = ascii.get(diff) {
            if diff >= s.len() || !right.is_ascii_digit() {
                break;
            }
            diff += 1;
        }

        s[start + 1..diff].parse().ok()
    }

    fn symbol_indexes(s: &str) -> impl Iterator<Item = usize> + '_ {
        s.chars().enumerate().filter_map(|(idx, c)| {
            if c != '.' && !c.is_ascii_digit() {
                Some(idx)
            } else {
                None
            }
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_inline_collect() {
            let cases = [
                ("617*......", vec![617]),
                ("617*123...", vec![617, 123]),
                ("...*123...", vec![123]),
                ("......*123", vec![123]),
                ("......*12.", vec![12]),
            ];

            for (idx, (input, expect)) in cases.into_iter().enumerate() {
                assert_eq!(inline_collect(input), expect, "Test #{}", idx + 1);
            }
        }

        #[test]
        fn test_compare_collect() {
            let case1 = ["467..114..", "...*......"];
            let case2 = ["..35..633.", "......#..."];
            let case3 = ["...*......", "..35..633."];

            assert_eq!(compare_collect(case1[0], case1[1]), vec![467]);
            assert_eq!(compare_collect(case2[0], case2[1]), vec![633]);
            assert_eq!(compare_collect(case3[1], case3[0]), vec![35]);
        }
    }
}

mod part_two {
    use super::part_one::*;
    use super::*;

    pub fn solution() {
        let inline = inline_ratios(INPUT);
        let up_below = up_below_ratios(INPUT);

        let input: Box<[&str]> = INPUT.lines().collect();
        let inline_other = input.windows(2).map(|window| {
            let [a, b] = window else {
                panic!("[ERROR] total lines from input is not divisible by 2");
            };

            inline_other_line_ratios(a, b)
                .into_iter()
                .chain(inline_other_line_ratios(b, a))
                .collect::<Vec<u32>>()
        });

        let other_line = input.windows(2).map(|window| {
            let [a, b] = window else {
                panic!("[ERROR] total lines from input is not divisible by 2");
            };

            other_line_ratios(a, b)
                .into_iter()
                .chain(other_line_ratios(b, a))
                .collect::<Vec<u32>>()
        });

        let a = inline_other
            .chain(other_line)
            .filter(|collected| !collected.is_empty())
            .reduce(|mut a, mut b| {
                a.append(&mut b);
                a
            })
            .unwrap_or_default();

        let sum: u32 = a.into_iter().chain(inline).chain(up_below).sum();

        println!("[PART 2 RESULT] {sum:#?}");
    }

    fn inline_ratios(s: &str) -> Vec<u32> {
        let mut ratios = Vec::new();

        for symdx in star_indexes(s) {
            let left = parse_left(s, symdx);
            let right = parse_right(s, symdx);

            if let (Some(left), Some(right)) = (left, right) {
                ratios.push(left * right);
            }
        }

        ratios
    }

    fn inline_other_line_ratios(a: &str, b: &str) -> Vec<u32> {
        let mut ratios = Vec::new();

        for symdx in star_indexes(b) {
            // parse inline
            let left = b.as_bytes().get(symdx - 1).filter(|c| c.is_ascii_digit());
            let right = b.as_bytes().get(symdx + 1).filter(|c| c.is_ascii_digit());

            let first_part = match (left, right) {
                (None, Some(_)) => parse_right(b, symdx),
                (Some(_), None) => parse_left(b, symdx),
                (Some(_), Some(_)) => b[symdx - 1..=symdx + 1].parse().ok(),
                (None, None) => b[symdx..=symdx].parse().ok(),
            };

            // parse other line
            let mut second_part = None;

            let left = parse_left(a, symdx);
            let right = parse_right(a, symdx);

            if left.is_some() {
                second_part = left;
            } else if right.is_some() {
                second_part = right;
            }

            if let (Some(first), Some(second)) = (first_part, second_part) {
                ratios.push(first * second);
            }
        }

        ratios
    }

    fn other_line_ratios(a: &str, b: &str) -> Vec<u32> {
        let mut ratios = Vec::new();

        for symdx in star_indexes(b) {
            let left = parse_left(a, symdx);
            let right = parse_right(a, symdx);

            if let (Some(left), Some(right)) = (left, right) {
                ratios.push(left * right);
            }
        }

        ratios
    }

    fn up_below_ratios(s: &str) -> Vec<u32> {
        let mut ratios = vec![];
        let line_len = s.lines().next().map(|line| line.len()).unwrap();

        for symdex in star_indexes(s) {
            let up_ratio = parse_non_inline(s, |diff| symdex.checked_sub(line_len + diff));

            let down_ratio = parse_non_inline(s, |diff| {
                let idx = symdex + (line_len + diff);
                if idx >= line_len {
                    Some(idx)
                } else {
                    None
                }
            });

            if let (Some(up), Some(down)) = (up_ratio, down_ratio) {
                ratios.push(up * down);
            }
        }

        ratios
    }

    fn star_indexes(s: &str) -> impl Iterator<Item = usize> + '_ {
        s.chars()
            .enumerate()
            .filter_map(|(idx, c)| if c == '*' { Some(idx) } else { None })
    }

    fn parse_non_inline(s: &str, idx_func: impl Fn(usize) -> Option<usize>) -> Option<u32> {
        let mut ratio = None;

        for diff in (0..=2).rev() {
            if let Some(idx) = idx_func(diff) {
                let digit = s.as_bytes().get(idx).filter(|c| c.is_ascii_digit());

                if digit.is_some() {
                    let left = s.as_bytes().get(idx - 1).filter(|c| c.is_ascii_digit());
                    let right = s.as_bytes().get(idx + 1).filter(|c| c.is_ascii_digit());

                    ratio = match (left, right) {
                        (None, Some(_)) => parse_right(s, idx - 1),
                        (Some(_), None) => parse_left(s, idx + 1),
                        (Some(_), Some(_)) => s[idx - 1..=idx + 1].parse().ok(),
                        (None, None) => s[idx..=idx].parse().ok(),
                    };

                    break;
                }
            }
        }

        ratio
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_inline_ratios() {
            let cases = [
                ("617*......", vec![]),
                ("617*123...", vec![617 * 123]),
                ("...*123...", vec![]),
                ("...123*123", vec![123 * 123]),
                (".....4*12.", vec![4 * 12]),
                (".....4*1..", vec![4]),
            ];

            for (idx, (has, expect)) in cases.into_iter().enumerate() {
                assert_eq!(inline_ratios(has), expect, "Test #{}", idx + 1);
            }
        }

        #[test]
        fn test_other_line_ratios() {
            let case1 = ["..35.633..", "....*.#..."];
            let case2 = ["....*.#...", "..35.633.."];

            assert_eq!(other_line_ratios(case1[0], case1[1]), vec![35 * 633]);
            assert_eq!(other_line_ratios(case2[1], case2[0]), vec![35 * 633]);
        }

        #[test]
        fn test_up_below_ratios() {
            let case1 =
"............................922........................./.173....894................*.........$....................177.......#..............
......................................................423....*....*.....816.........13.................=..............................251...
..154...................878*......................568......723..329....&......928.................-71.123.378..100...................*......"
            ;

            let case2 =
"...-....123..927.600...........-........*395........226./....676*755...........30..........58.....582.........671.118..43.367...943......478
.....................725........74...............................................+...........................-......*..*................&...
........378......563*.....*999............*...................497.349...26.....$............362.........&........723....20....717...317.....";

            let case3 =
"............................922........................./.173....894................*.........$....................177.......#..............
......................................................423....*....*.....816.........13.................=..............................251...
..154...................878*......................568......723..329....&......928.................-71.123.378..100...................*......
...-....123..927.600...........-........*395........226./....6..*.55...........30..........58.....582.........671.118..43.367...943......478
.....................725........74...............................................+...........................-......*..*................&...
........378......563*.....*999............*...................497.349...26.....$............362.........&........723....20....717...317.....";

            let case4 = "617.......
...*.+.58.
123..+.58.";

            assert_eq!(up_below_ratios(case1), vec![173 * 723, 894 * 329]);
            assert_eq!(up_below_ratios(case2), vec![118 * 723, 43 * 20]);
            assert_eq!(
                up_below_ratios(case3),
                vec![173 * 723, 894 * 329, 118 * 723, 43 * 20]
            );
            assert_eq!(up_below_ratios(case4), vec![617 * 123]);
        }

        #[test]
        fn test_inline_other_line_ratios() {
            let case1 = ["..35......", "....*633.."];
            let case2 = [".....35...", "....*633.."];
            let case3 = [".....35...", ".633*....."];
            let case4 = [
"............................922........................./.173....894................*.........$....................177.......#..............",
"................649*32...84.#...................315...................37............398.656...631..........340.985*..........522............",
            ];

            let case5 = [".....35...", ".633*....."];

            assert_eq!(inline_other_line_ratios(case1[0], case1[1]), vec![35 * 633]);
            assert_eq!(inline_other_line_ratios(case2[0], case2[1]), vec![35 * 633]);
            assert_eq!(inline_other_line_ratios(case3[0], case3[1]), vec![35 * 633]);
            assert_eq!(
                inline_other_line_ratios(case4[0], case4[1]),
                vec![985 * 177]
            );
            assert_eq!(inline_other_line_ratios(case5[0], case5[1]), vec![35 * 633]);
        }
    }
}
