use std::fs;

use anyhow::Result;

const DIGITS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day1.txt")?;

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let str_digits: String = line.chars().filter(|c| c.is_digit(10)).collect();
            extract_calibration_value(str_digits)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let str_digits = line_to_str_digits(line);
            extract_calibration_value(str_digits)
        })
        .sum()
}

fn extract_calibration_value(str_digits: String) -> u32 {
    let first_digit = str_digits.chars().next().expect("No first digit");
    let last_digit = str_digits.chars().last().expect("No second digit");

    format!("{}{}", first_digit, last_digit)
        .parse::<u32>()
        .unwrap()
}

fn line_to_str_digits(line: &str) -> String {
    let mut digits = String::new();
    let mut digits_index = Vec::new();

    for digit in DIGITS {
        let digit_index: Vec<(usize, &str)> = line.match_indices(digit).collect();
        digits_index.extend(digit_index);
    }

    digits_index.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, digit) in digits_index {
        match digit {
            "one" => digits.push('1'),
            "two" => digits.push('2'),
            "three" => digits.push('3'),
            "four" => digits.push('4'),
            "five" => digits.push('5'),
            "six" => digits.push('6'),
            "seven" => digits.push('7'),
            "eight" => digits.push('8'),
            "nine" => digits.push('9'),
            _ => digits.push_str(digit),
        }
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
                     pqr3stu8vwx
                     a1b2c3d4e5f
                     treb7uchet";

        let result = part1(&input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
                     eightwothree
                     abcone2threexyz
                     xtwone3four
                     4nineeightseven2
                     zoneight234
                     7pqrstsixteen";

        let result = part2(&input);
        assert_eq!(result, 281);
    }
}
