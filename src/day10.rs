use good_lp::{microlp, variable, variables, Expression, Solution, SolverModel};
use itertools::Itertools;
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
    if aoc.goal.iter().all(|&b| !b) {
        return 0;
    }
    for choices in 1..=aoc.buttons.len() {
        for button_set in aoc.buttons.iter().combinations(choices) {
            let mut lights = vec![false; aoc.goal.len()];
            for button in button_set {
                for &pos in button {
                    if pos < lights.len() {
                        lights[pos] = !lights[pos];
                    }
                }
            }
            if lights == aoc.goal {
                return choices;
            }
        }
    }
    unreachable!()
}

#[aoc(day10, part1)]
fn part1(input: &[AocType]) -> usize {
    input.iter().map(min_presses_to_goal).sum()
}

fn min_presses_to_joltage(aoc: &AocType) -> usize {
    let mut vars = variables!();
    let presses = (0..aoc.buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect_vec();

    let total_presses: Expression = presses.iter().sum();

    let mut problem = vars.minimise(total_presses).using(microlp);

    // for each jolt counter, sum of relevant presses must equal the target joltage
    for (jolt_idx, &target) in aoc.joltage.iter().enumerate() {
        let mut expr = Expression::from(0.0);

        for (btn_idx, relevant_idxs) in aoc.buttons.iter().enumerate() {
            // if button is relevant, add its press variable to the constraint
            if relevant_idxs.contains(&jolt_idx) {
                expr += presses[btn_idx];
            }
        }

        // sum of relevant presses == target joltage
        problem.add_constraint(expr.eq(target as f64));
    }

    let solution = problem.solve().unwrap();

    presses
        .iter()
        .map(|v| solution.value(*v).round() as usize)
        .sum()
}

// fn min_presses_to_joltage(aoc: &AocType) -> usize {
//     let button_ints = (0..aoc.buttons.len())
//         .map(|i| Int::new_const(format!("b{}", i)))
//         .collect_vec();

//     let targets: Vec<Vec<usize>> = (0..aoc.joltage.len())
//         .map(|i| {
//             aoc.buttons
//                 .iter()
//                 .enumerate()
//                 .filter_map(|(j, btn)| if btn.contains(&i) { Some(j) } else { None })
//                 .collect()
//         })
//         .collect();

//     let solver = Solver::new();
//     for (target, joltage) in targets.iter().zip(aoc.joltage.iter()) {
//         // sum for all buttons controlling a joltage asserts eq to joltage target
//         let s: Int = target.iter().filter_map(|&btn_idx| button_ints.get(btn_idx)).sum();
//         solver.assert(s.eq(*joltage as u64))
//     }
//     for b in &button_ints {
//         solver.assert(b.ge(&Int::from_i64(0)));
//     }
//     for (int, button) in button_ints.iter().zip(aoc.buttons.iter()) {
//         let min = button.iter().map(|i| aoc.joltage[*i]).min().unwrap();
//         solver.assert(int.le(min as u64));
//     }
//     let optimizer = Optimize::new();
//     optimizer.minimize(&button_ints.iter().sum::<Int>());
//     Model::of_optimize(&optimizer).unwrap();

//     let mut min = usize::MAX;
//     for solve in solver.solutions(button_ints, false).take(100) {
//         let sum: u64= solve.iter().map(Int::as_u64).map(|v| v.unwrap()).sum();
//         min = min.min(sum as usize);
//     }

//     min
// }

#[aoc(day10, part2)]
fn part2(input: &[AocType]) -> usize {
    input.iter().map(min_presses_to_joltage).sum()
}

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

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT).unwrap()), 33);
    }

    #[test]
    fn test_each_input_joltage() {
        let inputs = input_generator(TEST_INPUT).unwrap();
        assert_eq!(min_presses_to_joltage(&inputs[0]), 10);
        assert_eq!(min_presses_to_joltage(&inputs[1]), 12);
        assert_eq!(min_presses_to_joltage(&inputs[2]), 11);
    }
}
