use nom::{
    branch::alt,
    character::complete::{self, newline},
    combinator::value,
    multi::separated_list1,
    Parser,
};

use crate::convert_iresult_to_owned;

type AocType = (i64, i64); // factor, value

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let ret = separated_list1(
        newline,
        (
            alt((
                value(-1i64, complete::char('L')),
                value(1i64, complete::char('R')),
            )),
            complete::i64,
        ),
    )
    .parse(input);

    convert_iresult_to_owned(ret)
}

#[aoc(day01, part1)]
fn part1(input: &[AocType]) -> usize {
    let mut dial = 50;
    input
        .iter()
        .map(|(factor, value)| {
            dial += factor * value;
            dial
        })
        .filter(|d| d % 100 == 0)
        .count()
}

fn normalizer(dial: i64) -> i64 {
    dial - (dial.rem_euclid(100))
}

fn zero_crossing(curr: i64, next: i64) -> u64 {
    let nc = normalizer(curr);
    let nn = normalizer(next);
    if curr.rem_euclid(100) == 0 && next < curr {
        nc.abs_diff(nn) / 100 - 1
    } else if next.rem_euclid(100) == 0 && next < curr {
        nc.abs_diff(nn) / 100 + 1
    } else {
        nc.abs_diff(nn) / 100
    }
}

#[aoc(day01, part2)]
fn part2(input: &[AocType]) -> u64 {
    let mut dial = 50;
    input
        .iter()
        .map(|(factor, value)| {
            let curr_dial = dial;
            dial += factor * value;
            zero_crossing(curr_dial, dial)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT).unwrap()), 6);
    }

    #[test]
    fn test_crossing() {
        assert_eq!(normalizer(-50), -100);
        assert_eq!(zero_crossing(150, 50), 1);
        assert_eq!(zero_crossing(-50, 50), 1);
        assert_eq!(zero_crossing(-950, 50), 10);
        assert_eq!(zero_crossing(0, -50), 0);
        assert_eq!(zero_crossing(0, 50), 0);
    }
}
