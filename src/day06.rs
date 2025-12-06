use std::collections::HashMap;

use nom::{
    branch::{alt, permutation},
    character::complete::{self, newline, space0, space1},
    combinator::{all_consuming, opt, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use crate::convert_iresult_to_owned;

type AocType = (Vec<usize>, Operator); // factor, value

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Operator {
    Product,
    Sum,
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Sum, complete::char('+')),
        value(Operator::Product, complete::char('*')),
    ))
    .parse(input)
}

#[aoc_generator(day06, part1)]
fn input_generator_p1(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let intermidiate = all_consuming(terminated(
        separated_pair(
            separated_list1(
                newline,
                delimited(space0, separated_list1(space1, complete::usize), space0),
            ),
            newline,
            separated_list1(space1, parse_operator),
        ),
        permutation((opt(space1), opt(newline))),
    ))
    .parse(input);

    let (numbers, operators) = convert_iresult_to_owned(intermidiate)?;

    Ok(operators
        .into_iter()
        .enumerate()
        .map(|(i, o)| (numbers.iter().map(|n| n[i]).collect(), o))
        .collect())
}

#[aoc_generator(day06, part2)]
fn input_generator_p2(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let mut lines: Vec<_> = input.lines().collect();
    let last_row = lines.pop().unwrap();

    let nr_of_number_lines = lines.len();
    let max_columns = last_row.len();

    let pos_values: HashMap<_, _> = lines
        .into_iter()
        .enumerate()
        .flat_map(|(ln, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(col, c)| Some(((col, ln), c.to_digit(10)? as usize)))
        })
        .collect();

    let mut iter = last_row
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .peekable();

    let mut v: Vec<AocType> = Vec::new();

    while let Some((pos, operator)) = iter.next() {
        let col_max = iter.peek().map(|(c, _)| c - 1).unwrap_or(max_columns);
        let mut numbers = Vec::new();
        for c in pos..col_max {
            let mut number = 0;
            for row in 0..nr_of_number_lines {
                if let Some(d) = pos_values.get(&(c, row)) {
                    number *= 10;
                    number += d;
                }
            }
            assert_ne!(number, 0);
            numbers.push(number);
        }
        let o = convert_iresult_to_owned(parse_operator(&format!("{operator}")))?;

        v.push((numbers, o));
    }

    Ok(v)
}

#[aoc(day06, part1)]
fn part1(input: &[AocType]) -> usize {
    input
        .iter()
        .map(|(v, o)| {
            let iter = v.iter().cloned();
            match o {
                Operator::Product => iter.product::<usize>(),
                Operator::Sum => iter.sum(),
            }
        })
        .sum()
}

#[aoc(day06, part2)]
fn part2(input: &[AocType]) -> usize {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  
    "};

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator_p1(TEST_INPUT).unwrap()), 4277556)
    }

    #[test]
    fn test_p2() {
        assert_eq!(part1(&input_generator_p2(TEST_INPUT).unwrap()), 3263827)
    }
}
