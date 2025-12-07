use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct AocType {
    splitters: HashSet<(usize, usize)>,
    start: (usize, usize),
    size: (usize, usize),
}

#[aoc_generator(day07)]
fn input_generator(input: &str) -> AocType {
    let size = {
        let mut lines = input.lines();
        let height = lines.clone().count();
        let width = lines.next().map(|line| line.len()).unwrap();
        (width, height)
    };
    let mut lines = input.lines().enumerate();
    let start = lines
        .next()
        .map(|(_, line)| line)
        .unwrap()
        .char_indices()
        .find(|&(_, c)| c == 'S')
        .map(|(i, _)| (i, 0))
        .unwrap();

    let splitters = lines
        .flat_map(|(line_num, line)| {
            line.char_indices().filter_map(
                move |(i, c)| {
                    if c == '^' {
                        Some((i, line_num))
                    } else {
                        None
                    }
                },
            )
        })
        .collect();

    AocType {
        splitters,
        start,
        size,
    }
}

#[aoc(day07, part1)]
fn part1(input: &AocType) -> usize {
    let mut splits = 0;
    let mut beams = HashSet::new();
    beams.insert(input.start.0);
    for y in 1..input.size.1 {
        let mut new_beams = HashSet::new();
        for &beam in &beams {
            if input.splitters.contains(&(beam, y)) {
                splits += 1;
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }

    splits
}

#[aoc(day07, part2)]
fn part2(input: &AocType) -> usize {
    let mut beams = HashMap::new();
    beams.insert(input.start.0, 1);
    for y in 1..input.size.1 {
        let mut new_beams = HashMap::new();
        for (beam, amount) in &beams {
            if input.splitters.contains(&(*beam, y)) {
                new_beams
                    .entry(beam + 1)
                    .and_modify(|e| *e += amount)
                    .or_insert(*amount);
                new_beams
                    .entry(beam - 1)
                    .and_modify(|e| *e += amount)
                    .or_insert(*amount);
            } else {
                new_beams
                    .entry(*beam)
                    .and_modify(|e| *e += amount)
                    .or_insert(*amount);
            }
        }
        beams = new_beams;
    }

    beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn test_p1() {
        assert_eq!(part1(&input_generator(TEST_INPUT)), 21);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&input_generator(TEST_INPUT)), 40);
    }
}
