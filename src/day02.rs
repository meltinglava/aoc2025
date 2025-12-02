use std::ops::RangeInclusive;

use nom::{
    character::complete::{self},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};

use crate::convert_iresult_to_owned;

type AocType = RangeInclusive<usize>; // factor, value

#[aoc_generator(day02)]
pub fn input_generator(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let ret = separated_list1(
        complete::char(','),
        map(
            separated_pair(complete::usize, complete::char('-'), complete::usize),
            |(a, b)| a..=b,
        ),
    )
    .parse(input);

    convert_iresult_to_owned(ret)
}

fn id_validator(input: &usize) -> bool {
    let digits = input.ilog10() + 1;
    if digits % 2 == 1 {
        // odd number of digits
        return false;
    }
    let split = digits / 2;
    let factor = 10usize.pow(split);
    input / factor == input % factor
}

#[aoc(day02, part1)]
fn part1(input: &[AocType]) -> usize {
    input.iter().cloned().flatten().filter(id_validator).sum()
}

fn p2_id_validator(input: &usize) -> bool {
    let digits = input.ilog10() + 1;
    'digit_loop: for digit_factor in 1..=digits / 2 {
        if !digits.is_multiple_of(digit_factor) {
            continue;
        }
        let factor = 10usize.pow(digit_factor);
        let mut input = *input;
        let possible_repeated = input % factor;
        input -= possible_repeated;
        while input != 0 {
            input /= factor;
            if input % factor != possible_repeated {
                continue 'digit_loop;
            }
            input -= possible_repeated;
        }
        return true;
    }
    false
}

#[aoc(day02, part2)]
fn part2(input: &[AocType]) -> usize {
    input
        .iter()
        .cloned()
        .flatten()
        .filter(p2_id_validator)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_id_validator() {
        assert!(id_validator(&11));
        assert!(id_validator(&1188511885));
    }

    #[test]
    fn test_p2_id_validator() {
        assert!(p2_id_validator(&11));
        assert!(p2_id_validator(&999));
        assert!(p2_id_validator(&824824824));
        assert!(p2_id_validator(&1188511885));
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT).unwrap()), 1227775554)
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT).unwrap()), 4174379265)
    }
}
