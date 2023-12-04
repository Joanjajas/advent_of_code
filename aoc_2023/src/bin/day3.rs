use std::fs;

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
struct Number {
    row: i32,
    start: i32,
    end: i32,
    value: i32,
    visited: bool,
}

#[derive(Debug)]
struct Symbol<'a> {
    row: i32,
    col: i32,
    adjacent_nums: Vec<i32>,
    value: &'a str,
}

impl<'a> Symbol<'a> {
    fn add_adjacent(&mut self, num: i32) {
        if self.value == "*" {
            self.adjacent_nums.push(num);
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day3.txt")?;

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (mut numbers, symbols) = parse_input(input);

    symbols
        .iter()
        .map(|s| {
            numbers
                .iter_mut()
                .filter(|n| is_adjacent(s, n) && n.visited == false)
                .map(|n| {
                    n.visited = true;
                    n.value
                })
                .sum::<i32>()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let (mut numbers, mut symbols) = parse_input(input);

    for symbol in symbols.iter_mut() {
        for number in numbers.iter_mut() {
            if is_adjacent(symbol, number) && number.visited == false {
                symbol.add_adjacent(number.value);
                number.visited = true;
            }
        }
    }

    symbols
        .iter()
        .filter(|x| x.adjacent_nums.len() == 2)
        .map(|x| x.adjacent_nums.iter().fold(1, |acc, x| acc * x))
        .sum()
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    let number_re = Regex::new(r"\d+").unwrap();
    let symbol_re = Regex::new(r"[^\d.]+").unwrap();

    for (line_num, line) in input.lines().enumerate() {
        for mat in number_re.find_iter(line) {
            let number = Number {
                row: line_num as i32,
                start: mat.start() as i32,
                end: (mat.end() - 1) as i32,
                value: mat.as_str().parse().unwrap_or(0),
                visited: false,
            };

            numbers.push(number);
        }

        for mat in symbol_re.find_iter(line) {
            let symbol = Symbol {
                row: line_num as i32,
                col: mat.start() as i32,
                adjacent_nums: vec![],
                value: mat.as_str(),
            };

            symbols.push(symbol);
        }
    }

    (numbers, symbols)
}

fn is_adjacent(a: &Symbol, b: &Number) -> bool {
    let same_row = a.row == b.row;
    let same_col = a.col == b.start || a.col == b.end;
    let adjacent_row = a.row == b.row - 1 || a.row == b.row + 1;
    let adjacent_col =
        a.col == b.start - 1 || a.col == b.end + 1 || a.col == b.start + 1 || a.col == b.end - 1;

    same_row && adjacent_col || same_col && adjacent_row || adjacent_row && adjacent_col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";

        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part2() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";

        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
