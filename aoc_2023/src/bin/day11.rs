use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day11.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> usize {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);

        x_diff + y_diff
    }
}

fn part_1(input: &str) -> usize {
    let galaxies = galaxy_map(input, 2);

    galaxies
        .iter()
        .map(|galaxy| {
            galaxies
                .iter()
                .fold(0, |acc, other_galaxy| acc + galaxy.distance(other_galaxy))
        })
        .sum::<usize>()
        / 2
}

fn part_2(input: &str) -> usize {
    let galaxies = galaxy_map(input, 1000000);

    galaxies
        .iter()
        .map(|galaxy| {
            galaxies
                .iter()
                .fold(0, |acc, other_galaxy| acc + galaxy.distance(other_galaxy))
        })
        .sum::<usize>()
        / 2
}

fn galaxy_map(input: &str, expansion: usize) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();

    // expand verticaly
    let mut vertical_expansion = 0;
    for (y, line) in input.lines().enumerate() {
        let mut expand = true;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy {
                    x,
                    y: y + vertical_expansion,
                });
                expand = false;
            }
        }

        if expand {
            vertical_expansion += expansion - 1;
        }
    }

    // expand horizontaly
    let mut horizontal_expansion = 0;
    for y in 0..input.lines().count() {
        let mut expand = true;
        for x in 0..input.lines().count() {
            if input.lines().nth(x).unwrap().chars().nth(y).unwrap() == '#' {
                expand = false;
            }
        }

        if expand {
            galaxies
                .iter_mut()
                .filter(|galaxy| galaxy.x > y + horizontal_expansion)
                .for_each(|galaxy| galaxy.x += expansion - 1);
            horizontal_expansion += expansion - 1;
        }
    }

    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = fs::read_to_string("input/test/day11.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 374);

        Ok(())
    }

    #[test]
    fn kest_part2() -> Result<()> {
        let input = fs::read_to_string("input/test/day11.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 82000210);

        Ok(())
    }
}
