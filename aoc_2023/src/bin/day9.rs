use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day9.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part_1(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let history = generate_history(line);

        sum += history
            .iter()
            .rev()
            .skip(1)
            .fold(0, |acc, x| acc + x.last().unwrap());
    }

    sum
}

fn part_2(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let history = generate_history(line);

        sum += history
            .iter()
            .rev()
            .skip(1)
            .fold(0, |acc, x| x.first().unwrap() - acc);
    }

    sum
}

fn generate_history(input: &str) -> Vec<Vec<i64>> {
    let mut history: Vec<Vec<i64>> = Vec::new();

    let mut history_step: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    history.push(history_step.clone());

    while !history_step.iter().all(|x| x == &0) {
        history_step = history_step.windows(2).map(|w| w[1] - w[0]).collect();
        history.push(history_step.clone());
    }

    history
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input = fs::read_to_string("input/test/day9.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 114);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = fs::read_to_string("input/test/day9.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 2);

        Ok(())
    }
}
