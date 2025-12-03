use nom::{
    bytes::complete::take,
    character::complete::{self, newline},
    combinator::{all_consuming, map_parser, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    Parser,
};

use crate::convert_iresult_to_owned;

type AocType = Vec<usize>; // factor, value

#[aoc_generator(day03)]
pub fn input_generator(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let ret = all_consuming(terminated(
        separated_list1(newline, many1(map_parser(take(1usize), complete::usize))),
        opt(newline),
    ))
    .parse(input);

    convert_iresult_to_owned(ret)
}

fn find_joltage(input: &[usize], size: usize) -> usize {
    let mut to_start = 0;
    let mut accumulator = 0;
    let mut end = input.len() - size + 1;
    for _index in 0..size {
        accumulator *= 10;
        let mut max_value = 0;
        for (i, battery) in input.iter().enumerate().take(end).skip(to_start) {
            if *battery > max_value {
                max_value = *battery;
                to_start = i + 1;
            }
        }
        accumulator += max_value;
        end += 1;
    }
    accumulator
}

#[aoc(day03, part1)]
fn part1(input: &[AocType]) -> usize {
    input.iter().map(|line| find_joltage(line, 2)).sum()
}

#[aoc(day03, part2)]
fn part2(input: &[AocType]) -> usize {
    input.iter().map(|line| find_joltage(line, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT).unwrap()), 357);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT).unwrap()), 3121910778619);
    }

    #[test]
    fn test_line_p1() {
        let line = "987654321111111";
        assert_eq!(find_joltage(&input_generator(line).unwrap()[0], 2), 98);
    }

    #[test]
    fn test_line_p1_2() {
        let line = "811111111111119";
        assert_eq!(find_joltage(&input_generator(line).unwrap()[0], 2), 89);
    }
}
