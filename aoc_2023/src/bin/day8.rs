use std::collections::HashMap;
use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day8.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

struct NodeMap<'a> {
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> NodeMap<'a> {
    fn new(str_nodes: &'a str) -> Self {
        let mut nodes = HashMap::new();

        for line in str_nodes.lines() {
            let current_node = line.split(" = ").nth(0).unwrap();
            let (left_node, right_node) = line
                .split(" = ")
                .nth(1)
                .unwrap()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();

            nodes.insert(current_node, (left_node, right_node));
        }

        Self { nodes }
    }

    fn next_node(&self, current_node: &str, instruction: char) -> &str {
        let (left_node, right_node) = self.nodes.get(&current_node).unwrap();
        match instruction {
            'L' => left_node,
            'R' => right_node,
            _ => panic!("Invalid instruction"),
        }
    }
}

fn part_1(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let map = NodeMap::new(nodes);

    steps_to_end("AAA", &map, instructions)
}

fn part_2(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let map = NodeMap::new(nodes);

    let start_nodes = nodes
        .lines()
        .map(|line| line.split(" = ").next().unwrap())
        .filter(|line| line.ends_with('A'))
        .collect::<Vec<&str>>();

    start_nodes
        .iter()
        .map(|node| steps_to_end(node, &map, instructions))
        .reduce(least_common_multiple)
        .unwrap()
}

fn steps_to_end(start_node: &str, map: &NodeMap, instructions: &str) -> usize {
    let mut steps = 0;
    let mut current_node = start_node;

    while !current_node.ends_with('Z') {
        for instruction in instructions.chars() {
            current_node = map.next_node(current_node, instruction);
            steps += 1;
        }
    }

    steps
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greater_common_divisor(a, b)
}

fn greater_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    greater_common_divisor(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input_1 = fs::read_to_string("input/test/day8_part_1_1.txt")?;
        let input_2 = fs::read_to_string("input/test/day8_part_1_2.txt")?;

        let result_1 = part_1(&input_1);
        let result_2 = part_1(&input_2);

        assert_eq!(result_1, 2);
        assert_eq!(result_2, 6);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = fs::read_to_string("input/test/day8_part_2.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 6);

        Ok(())
    }
}
