use nom::{
    branch::alt,
    character::complete::{self, space1},
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

use crate::convert_iresult_to_owned;

#[derive(Debug, Clone)]
struct AocType {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse_comma_separated_usize(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(complete::char(','), complete::usize).parse(input)
}

fn light_goal_parser(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(
        complete::char('['),
        many1(alt((
            complete::char('#').map(|_| true),
            complete::char('.').map(|_| false),
        ))),
        complete::char(']'),
    )
    .parse(input)
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let ret = separated_list1(
        complete::newline,
        (
            light_goal_parser,
            delimited(
                space1,
                separated_list1(
                    space1,
                    delimited(
                        complete::char('('),
                        parse_comma_separated_usize,
                        complete::char(')'),
                    ),
                ),
                space1,
            ),
            delimited(
                complete::char('{'),
                parse_comma_separated_usize,
                complete::char('}'),
            ),
        )
            .map(|(lights, buttons, joltage)| AocType {
                goal: lights,
                buttons,
                joltage,
            }),
    )
    .parse(input);

    convert_iresult_to_owned(ret)
}

fn min_presses_to_goal(aoc: &AocType) -> usize {
    todo!()
}

#[aoc(day10, part1)]
fn part1(input: &[AocType]) -> usize {
    input.iter().map(min_presses_to_goal).sum()
}

// #[aoc(day10, part2)]
// fn part2(input: &[AocType]) -> usize {
//    todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn test_parse() {
        let p = input_generator(TEST_INPUT).unwrap();
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT).unwrap()), 7);
    }

    #[test]
    fn test_each_input() {
        let inputs = input_generator(TEST_INPUT).unwrap();
        assert_eq!(min_presses_to_goal(&inputs[0]), 2);
        assert_eq!(min_presses_to_goal(&inputs[1]), 3);
        assert_eq!(min_presses_to_goal(&inputs[2]), 2);
    }
}
