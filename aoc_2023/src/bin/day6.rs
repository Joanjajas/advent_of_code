use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day6.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part_1(input: &str) -> u64 {
    let (race_durations, record_distances) = parse_input(input);
    let mut possible_wins = vec![0; race_durations.len()];

    for (index, (race_duration, record_distance)) in
        race_durations.iter().zip(record_distances).enumerate()
    {
        possible_wins[index] = calculate_possible_wins(*race_duration, record_distance);
    }

    possible_wins.iter().fold(1, |acc, x| acc * x)
}

fn part_2(input: &str) -> u64 {
    let (race_durations, record_distances) = parse_input(input);

    let race_duration = race_durations
        .iter()
        .map(|x| x.to_string())
        .fold(String::from(""), |mut acc, x| {
            acc.push_str(&x);
            acc
        })
        .parse()
        .unwrap();

    let record_distance: u64 = record_distances
        .iter()
        .map(|x| x.to_string())
        .fold(String::from(""), |mut acc, x| {
            acc.push_str(&x);
            acc
        })
        .parse()
        .unwrap();

    calculate_possible_wins(race_duration, record_distance)
}

fn calculate_possible_wins(race_duration: u64, record_distance: u64) -> u64 {
    let left_index = (0..race_duration)
        .into_iter()
        .find(|i| i * (race_duration - i) > record_distance)
        .unwrap();

    let right_index = (0..race_duration)
        .into_iter()
        .rev()
        .find(|i| i * (race_duration - i) > record_distance)
        .unwrap();

    right_index - left_index + 1
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let race_durations: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("Time: ")
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let record_distances: Vec<u64> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .trim_start_matches("Distance: ")
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    (race_durations, record_distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = fs::read_to_string("input/test/day6.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 288);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = fs::read_to_string("input/test/day6.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 71503);

        Ok(())
    }
}
