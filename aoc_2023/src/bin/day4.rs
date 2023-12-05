use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day4.txt")?;

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1(input: &str) -> u32 {
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

fn part2(input: &str) -> u32 {
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
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part2(input);
        assert_eq!(result, 30);
    }
}
