use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use petgraph::prelude::DiGraphMap;

type Graph<'a> = DiGraphMap<&'a str, ()>;

fn parse_tree<'a>(input: &'a str) -> IResult<&'a str, Graph<'a>> {
    let mut graph = DiGraphMap::new();

    let values = all_consuming(terminated(
        separated_list1(
            newline,
            separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
        ),
        opt(newline),
    ))
    .parse(input)?;

    for (parent, children) in values.1 {
        graph.add_node(parent);
        for child in children {
            graph.add_node(child);
            graph.add_edge(parent, child, ());
        }
    }

    Ok((input, graph))
}

fn traverse_p1<'a, 'b>(node: &'a str, graph: &Graph<'b>) -> usize
where
    'a: 'b,
{
    if node == "out" {
        return 1;
    }
    graph
        .neighbors(node)
        .map(|n| traverse_p1(n, graph))
        .sum::<usize>()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let (_, graph) = parse_tree(input).unwrap();

    traverse_p1("you", &graph)
}

fn traverse_p2<'a, 'b, 'c>(
    node: &'a str,
    mut dac: bool,
    mut fft: bool,
    graph: &Graph<'b>,
    dp: &mut HashMap<(&'c str, bool, bool), usize>,
) -> usize
where
    'a: 'b,
    'b: 'c,
{
    if let Some(&res) = dp.get(&(node, dac, fft)) {
        return res;
    }
    if node == "out" {
        return if dac && fft { 1 } else { 0 };
    } else if node == "dac" {
        dac = true;
    } else if node == "fft" {
        fft = true;
    }
    let ans = graph
        .neighbors(node)
        .map(|n| traverse_p2(n, dac, fft, graph, dp))
        .sum::<usize>();
    dp.insert((node, dac, fft), ans);
    ans
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let (_, graph) = parse_tree(input).unwrap();

    let mut dp = HashMap::new();

    traverse_p2("svr", false, false, &graph, &mut dp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const TEST_INPUT1: &str = indoc! {"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    "};

    const TEST_INPUT2: &str = indoc! {"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    "};

    #[test]
    fn test_parse() {
        let (_, graph) = parse_tree(TEST_INPUT1).unwrap();
        assert_eq!(graph.node_count(), 11);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(TEST_INPUT1), 5);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(TEST_INPUT2), 2);
    }
}
