use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    character::complete::{self, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::terminated,
    Parser,
};

use crate::convert_iresult_to_owned;

type AocType = (usize, usize, usize); // factor, value

#[aoc_generator(day08)]
pub fn input_generator(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let ret = all_consuming(terminated(
        separated_list1(
            newline,
            (
                terminated(complete::usize, complete::char(',')),
                terminated(complete::usize, complete::char(',')),
                complete::usize,
            ),
        ),
        opt(newline),
    ))
    .parse(input);

    convert_iresult_to_owned(ret)
}

fn squared_distance(a: &AocType, b: &AocType) -> usize {
    let x_diff = a.0.abs_diff(b.0);
    let y_diff = a.1.abs_diff(b.1);
    let z_diff = a.2.abs_diff(b.2);
    x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
}

fn connect_circuts(input: &[AocType], threshold: usize) -> usize {
    let values = input
        .iter()
        .combinations(2)
        .map(move |mut connection| {
            connection.sort();
            let (b, a) = (connection[1], connection[0]);
            (connection, squared_distance(a, b))
        })
        .sorted_unstable_by_key(|(_, dist)| *dist)
        .collect::<Vec<_>>();

    let mut id = 0;
    let mut circuits: HashMap<usize, Vec<AocType>> = HashMap::new();
    let mut boxes_to_circuits: HashMap<(usize, usize, usize), usize> = HashMap::new();
    for (pair, _) in values.into_iter().take(threshold) {
        let (a, b) = (
            boxes_to_circuits.get(pair[0]),
            boxes_to_circuits.get(pair[1]),
        );
        match (a, b) {
            (Some(a_id), Some(b_id)) => {
                let a_id = *a_id;
                let b_id = *b_id;
                if a_id != b_id {
                    for box_id in circuits.remove(&b_id).unwrap() {
                        boxes_to_circuits.insert(box_id, a_id);
                        circuits.get_mut(&a_id).unwrap().push(box_id);
                    }
                }
            }
            (Some(a_id), None) => {
                let a_id = *a_id;
                boxes_to_circuits.insert(*pair[1], a_id);
                circuits.get_mut(&a_id).unwrap().push(*pair[1]);
            }
            (None, Some(b_id)) => {
                let b_id = *b_id;
                boxes_to_circuits.insert(*pair[0], b_id);
                circuits.get_mut(&b_id).unwrap().push(*pair[0]);
            }
            (None, None) => {
                boxes_to_circuits.insert(*pair[0], id);
                boxes_to_circuits.insert(*pair[1], id);
                circuits.insert(id, vec![*pair[0], *pair[1]]);
                id += 1;
            }
        }
        if circuits.len() == 1 && circuits.values().next().unwrap().len() == input.len() {
            return pair[0].0 * pair[1].0; // part 2
        }
    }
    //part 1
    circuits
        .values()
        .map(|v| v.len())
        .sorted_unstable_by_key(|k| usize::MAX - *k)
        .take(3)
        .product()
}

#[aoc(day08, part1)]
fn part1(input: &[AocType]) -> usize {
    connect_circuts(input, 1000)
}

#[aoc(day08, part2)]
fn part2(input: &[AocType]) -> usize {
    connect_circuts(input, usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn test_p1() {
        let input = input_generator(TEST_INPUT).unwrap();
        let result = connect_circuts(&input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_p2() {
        let input = input_generator(TEST_INPUT).unwrap();
        let result = part2(&input);
        assert_eq!(result, 25272);
    }
}
