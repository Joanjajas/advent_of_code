use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day5.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Map {
    fn from_block(block: &str) -> Self {
        let ranges = block
            .lines()
            .skip(1)
            .map(|line| {
                let mut values = line.split_whitespace();
                let dest_range_start = values.next().unwrap().parse().unwrap();
                let source_range_start = values.next().unwrap().parse().unwrap();
                let range_length = values.next().unwrap().parse().unwrap();

                Range {
                    dest_range_start,
                    source_range_start,
                    range_length,
                }
            })
            .collect();

        Map { ranges }
    }

    fn map_value(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find(|range| {
                let lower_bound = range.source_range_start;
                let upper_bound = range.source_range_start + range.range_length;
                lower_bound <= value && value <= upper_bound
            })
            .map(|range| range.dest_range_start + (value - range.source_range_start))
            .unwrap_or(value)
    }
}

fn part_1(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);

    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.map_value(acc)))
        .min()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);
    let seed_ranges: Vec<&[u64]> = seeds.chunks(2).collect();

    seed_ranges
        .iter()
        .map(|range| {
            let mut min = std::u64::MAX;
            for seed in range[0]..=range[0] + range[1] {
                let mut value = seed;
                for map in &maps {
                    value = map.map_value(value);
                }

                min = std::cmp::min(min, value);
            }

            min
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut blocks = input.split("\n\n");
    let seeds = blocks
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps = blocks.map(|block| Map::from_block(block)).collect();

    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = fs::read_to_string("input/test/day5.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 35);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = fs::read_to_string("input/test/day5.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 46);

        Ok(())
    }
}
