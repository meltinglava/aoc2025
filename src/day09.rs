use itertools::Itertools;
use nom::{
    character::complete::{self, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Parser,
};

use crate::convert_iresult_to_owned;

type AocType = (usize, usize); // factor, value
type Edge = (AocType, AocType);

#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> Result<Vec<AocType>, nom::Err<nom::error::Error<String>>> {
    let ret = all_consuming(terminated(
        separated_list1(
            newline,
            separated_pair(complete::usize, complete::char(','), complete::usize),
        ),
        opt(newline),
    ))
    .parse(input);

    convert_iresult_to_owned(ret)
}

fn squared_area(a: &AocType, b: &AocType) -> usize {
    let x_diff = a.0.abs_diff(b.0) + 1;
    let y_diff = a.1.abs_diff(b.1) + 1;
    x_diff * y_diff
}

#[aoc(day09, part1)]
fn part1(input: &[AocType]) -> usize {
    input
        .iter()
        .combinations(2)
        .map(|connection| squared_area(connection[0], connection[1]))
        .max()
        .unwrap()
}

fn is_fully_contained(edges: &[Edge], corner1: AocType, corner2: AocType) -> bool {
    let x_min = corner1.0.min(corner2.0);
    let x_max = corner1.0.max(corner2.0);
    let y_min = corner1.1.min(corner2.1);
    let y_max = corner1.1.max(corner2.1);

    for edge in edges {
        let edge_x_min = edge.0 .0.min(edge.1 .0);
        let edge_x_max = edge.0 .0.max(edge.1 .0);
        let edge_y_min = edge.0 .1.min(edge.1 .1);
        let edge_y_max = edge.0 .1.max(edge.1 .1);
        if x_min < edge_x_max && x_max > edge_x_min && y_min < edge_y_max && y_max > edge_y_min {
            return false;
        }
    }
    true
}

#[aoc(day09, part2)]
fn part2(red_tiles: &[AocType]) -> usize {
    let mut edges = red_tiles.windows(2).map(|w| (w[0], w[1])).collect_vec();

    edges.push((red_tiles[red_tiles.len() - 1], red_tiles[0]));

    let mut result = 0;

    for rectangle in red_tiles.iter().combinations(2) {
        let area = squared_area(rectangle[0], rectangle[1]);
        if area <= result {
            continue;
        }

        if is_fully_contained(&edges, *rectangle[0], *rectangle[1]) {
            result = area;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT).unwrap()), 50);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT).unwrap()), 24);
    }
}
