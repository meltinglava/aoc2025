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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    edge: [AocType; 2],
}

fn ccw(a: AocType, b: AocType, c: AocType) -> bool {
    let a_x = a.0 as isize;
    let a_y = a.1 as isize;
    let b_x = b.0 as isize;
    let b_y = b.1 as isize;
    let c_x = c.0 as isize;
    let c_y = c.1 as isize;
    (c_y - a_y) * (b_x - a_x) > (b_y - a_y) * (c_x - a_x)
}

fn intersect(a: &Edge, b: &Edge) -> bool {
    ccw(a.edge[0], b.edge[0], b.edge[1]) != ccw(a.edge[1], b.edge[0], b.edge[1])
        && ccw(a.edge[0], a.edge[1], b.edge[0]) != ccw(a.edge[0], a.edge[1], b.edge[1])
}

#[aoc(day09, part2)]
fn part2(red_tiles: &[AocType]) -> usize {
    let mut edges = red_tiles
        .windows(2)
        .map(|f| Edge { edge: [f[0], f[1]] })
        .collect_vec();

    edges.push(Edge {
        edge: [red_tiles[red_tiles.len() - 1], red_tiles[0]],
    });

    let mut max = 0;
    red_tiles.iter().combinations(2).for_each(|connection| {
        let area = squared_area(connection[0], connection[1]);
        if area <= max {
            return;
        }
        let connection_edges = [
            Edge {
                edge: [*connection[0], (connection[0].0, connection[1].1)],
            },
            Edge {
                edge: [(connection[0].0, connection[1].1), *connection[1]],
            },
            Edge {
                edge: [*connection[1], (connection[1].0, connection[0].1)],
            },
            Edge {
                edge: [(connection[1].0, connection[0].1), *connection[0]],
            },
        ];
        if !edges
            .iter()
            .any(|e| connection_edges.iter().any(|ce| intersect(e, ce)))
        {
            max = area;
        }
    });

    max
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

    const TEST_INPUT2: &str = indoc! {"
        1,0
        3,0
        3,6
        16,6
        16,0
        18,0
        18,9
        13,9
        13,7
        6,7
        6,9
        1,9
    "};

    #[test]
    fn test_intersect() {
        let edge1 = Edge {
            edge: [(1, 1), (4, 4)],
        };
        let edge2 = Edge {
            edge: [(1, 4), (4, 1)],
        };
        let edge3 = Edge {
            edge: [(5, 5), (6, 6)],
        };
        let edge4 = Edge {
            edge: [(1, 1), (1, 2)],
        };
        let edge5 = Edge {
            edge: [(0, 3), (3, 3)],
        };
        assert!(intersect(&edge1, &edge2));
        assert!(!intersect(&edge1, &edge3));
        assert!(!intersect(&edge1, &edge1));
        assert!(!intersect(&edge4, &edge5));
    }

    #[test]
    fn test_intersect2() {
        let edge1 = Edge {
            edge: [(1, 1), (5, 4)],
        };
        let edge2 = Edge {
            edge: [(1, 7), (9, 1)],
        };
        assert!(!intersect(&edge1, &edge2));
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT).unwrap()), 50);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT).unwrap()), 24);
    }

    #[test]
    fn test_p2_2() {
        assert_eq!(part2(&input_generator(TEST_INPUT2).unwrap()), 30);
    }
}
