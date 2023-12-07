use std::fs;

use anyhow::Result;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day2.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part_1(input: &str) -> u32 {
    let mut id_sum = 0;

    for line in input.lines() {
        let (id, subsets_list) = line.split_once(": ").unwrap();
        let subsets: Vec<&str> = subsets_list.split("; ").collect();

        let mut impossible = false;
        for subset in subsets {
            let plays: Vec<&str> = subset.split(", ").collect();

            for play in plays {
                let (quantity, color) = play.split_once(' ').unwrap();
                let quantity: u32 = quantity.parse().unwrap_or(0);

                match color {
                    "red" => {
                        if quantity > MAX_RED {
                            impossible = true
                        }
                    }
                    "green" => {
                        if quantity > MAX_GREEN {
                            impossible = true
                        }
                    }
                    "blue" => {
                        if quantity > MAX_BLUE {
                            impossible = true
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }

        if !impossible {
            id_sum += id
                .split_whitespace()
                .last()
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap_or(0);
        }
    }

    id_sum
}

fn part_2(input: &str) -> u32 {
    let mut power_sum = 0;

    for line in input.lines() {
        let (_, subsets_list) = line.split_once(": ").unwrap();
        let plays: Vec<&str> = subsets_list
            .split("; ")
            .flat_map(|play| play.split(", "))
            .collect();

        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;
        for play in plays {
            let (quantity, color) = play.split_once(' ').unwrap();
            let quantity: u32 = quantity.parse().unwrap_or(0);

            match color {
                "red" => {
                    red_min = std::cmp::max(red_min, quantity);
                }
                "green" => {
                    green_min = std::cmp::max(green_min, quantity);
                }
                "blue" => {
                    blue_min = std::cmp::max(blue_min, quantity);
                }
                _ => unreachable!(),
            }
        }

        let power = red_min * green_min * blue_min;
        power_sum += power;
    }

    power_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input = fs::read_to_string("input/test/day2.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 8);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = fs::read_to_string("input/test/day2.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 2286);

        Ok(())
    }
}
