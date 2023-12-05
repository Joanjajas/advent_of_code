use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day5.txt")?;

    let part1 = part1(&input);
    let part2 = part2(&input);

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

fn part1(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);

    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.map_value(acc)))
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
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
    fn test_part1() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48\n\
                     \n\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15\n\
                     \n\
                     fertilizer-to-water map:\n\
                     49 53 8\n\
                     0 11 42\n\
                     42 0 7\n\
                     57 7 4\n\
                     \n\
                     water-to-light map:\n\
                     88 18 7\n\
                     18 25 70\n\
                     \n\
                     light-to-temperature map:\n\
                     45 77 23\n\
                     81 45 19\n\
                     68 64 13\n\
                     \n\
                     temperature-to-humidity map:\n\
                     0 69 1\n\
                     1 0 69\n\
                     \n\
                     humidity-to-location map:\n\
                     60 56 37\n\
                     56 93 4";

        let result = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48\n\
                     \n\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15\n\
                     \n\
                     fertilizer-to-water map:\n\
                     49 53 8\n\
                     0 11 42\n\
                     42 0 7\n\
                     57 7 4\n\
                     \n\
                     water-to-light map:\n\
                     88 18 7\n\
                     18 25 70\n\
                     \n\
                     light-to-temperature map:\n\
                     45 77 23\n\
                     81 45 19\n\
                     68 64 13\n\
                     \n\
                     temperature-to-humidity map:\n\
                     0 69 1\n\
                     1 0 69\n\
                     \n\
                     humidity-to-location map:\n\
                     60 56 37\n\
                     56 93 4";

        let result = part2(input);
        assert_eq!(result, 46);
    }
}
