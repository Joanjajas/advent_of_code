use anyhow::Result;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/test/day12.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part_1(input: &str) -> usize {
    for line in input.lines() {
        let (springs, nums) = line.split_once(' ').unwrap();
    }

    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = fs::read_to_string("input/test/day12.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 21);

        Ok(())
    }

    #[test]
    fn kest_part2() -> Result<()> {
        let input = fs::read_to_string("input/test/day12.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 82000210);

        Ok(())
    }
}
