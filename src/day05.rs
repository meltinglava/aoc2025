use nom::{
    character::complete::{self, newline},
    combinator::{all_consuming, map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Parser,
};
use std::ops::RangeInclusive;

use crate::convert_iresult_to_owned;

type AocType = (Vec<RangeInclusive<usize>>, Vec<usize>); // ranges

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Result<AocType, nom::Err<nom::error::Error<String>>> {
    let ret = all_consuming(terminated(
        separated_pair(
            separated_list1(
                newline,
                map(
                    separated_pair(complete::usize, complete::char('-'), complete::usize),
                    |(a, b)| a..=b,
                ),
            ),
            (newline, newline),
            separated_list1(newline, complete::usize),
        ),
        opt(newline),
    ))
    .parse(input);

    convert_iresult_to_owned(ret)
}

#[aoc(day05, part1)]
fn part1(input: &AocType) -> usize {
    let (ranges, ids) = input;
    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

#[aoc(day05, part2)]
fn part2(input: &AocType) -> usize {
    let mut input = input.0.clone();
    input.sort_unstable_by_key(|n| *n.start());
    let mut index = 0;
    while index < input.len() - 1 {
        let start = *input[index].start();
        let end = *input[index].end();
        let i = index + 1;
        if *input[i].start() <= end {
            input[index] = start..=end.max(*input[i].end());
            input.remove(i);
            continue;
        }

        index += 1;
    }
    input.iter().map(|r| r.end() - r.start() + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT).unwrap()), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT).unwrap()), 14);
    }
}
