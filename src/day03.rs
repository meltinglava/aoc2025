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

fn find_joltage(input: &[usize]) -> usize {
    let mut current_high = 0;
    let mut position = 0;
    let mut iter = input.iter().enumerate().peekable();
    while let Some((i, joltage)) = iter.next() {
        if joltage > &current_high && iter.peek().is_some() {
            current_high = *joltage;
            position = i;
        }
    }
    let mut second_high = 0;
    for (i, joltage) in input.iter().enumerate().skip(position) {
        if joltage > &second_high && i != position {
            second_high = *joltage;
        }
    }
    current_high * 10 + second_high
}

#[aoc(day03, part1)]
fn part1(input: &[AocType]) -> usize {
    input.iter().map(|line| find_joltage(line)).sum()
}

// #[aoc(day03, part2)]
// fn part2(input: &[AocType]) -> usize {
//    todo!()
// }

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
}
