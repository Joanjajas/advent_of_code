use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day4.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (wining_numbers, my_numbers) = numbers.split_once(" | ").unwrap();

        let wining_numbers: Vec<u32> = wining_numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let my_numbers: Vec<u32> = my_numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let matches = my_numbers
            .iter()
            .filter(|n| wining_numbers.contains(n))
            .count() as u32;

        let points = match matches {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => u32::pow(2, matches - 1),
        };

        sum += points;
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let mut cards = vec![0; input.lines().count() + 1];

    for line in input.lines() {
        let (card_number, numbers) = line.split_once(": ").unwrap();
        let (wining_numbers, my_numbers) = numbers.split_once(" | ").unwrap();

        let card_number = card_number
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let wining_numbers: Vec<u32> = wining_numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let my_numbers: Vec<u32> = my_numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let matches = my_numbers
            .iter()
            .filter(|n| wining_numbers.contains(n))
            .count() as usize;

        cards[card_number] += 1;
        for i in 1..=matches {
            cards[card_number + i] += cards[card_number];
        }
    }

    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input = fs::read_to_string("input/test/day4.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 13);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = fs::read_to_string("input/test/day4.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 30);

        Ok(())
    }
}
