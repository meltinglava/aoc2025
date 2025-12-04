use std::collections::HashSet;

use crate::grid::Direction;

type AocType = (HashSet<(usize, usize)>, usize); // factor, value

#[aoc_generator(day04)]
pub fn input_generator(input: &str) -> AocType {
    let paper = input
        .lines()
        .enumerate()
        .flat_map(|(line_nr, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '@')
                .map(move |(col_nr, _)| (col_nr, line_nr))
        })
        .collect();
    let size = input.lines().next().unwrap().len();
    (paper, size)
}

#[aoc(day04, part1)]
fn part1(input: &AocType) -> usize {
    let (input, size) = input;
    input
        .iter()
        .copied()
        .filter(|pos| forklift_accessable(*pos, input, *size) < 4)
        .count()
}

fn forklift_accessable(pos: (usize, usize), input: &HashSet<(usize, usize)>, size: usize) -> usize {
    Direction::all()
        .into_iter()
        .filter_map(|dir| dir.step(pos, size))
        .filter(|new_pos| input.contains(new_pos))
        .count()
}

#[aoc(day04, part2)]
fn part2(input: &AocType) -> usize {
    let (input, size) = input;
    let mut input = input.clone();
    let mut removed = 0;
    loop {
        let to_remove = input
            .iter()
            .copied()
            .filter(|&pos| forklift_accessable(pos, &input, *size) < 4)
            .collect::<Vec<_>>();
        if to_remove.is_empty() {
            break;
        } else {
            for pos in to_remove {
                input.remove(&pos);
                removed += 1;
            }
        }
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT)), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT)), 43);
    }
}
