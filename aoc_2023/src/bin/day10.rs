use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day10.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileKind {
    Start,
    Ground,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl TileKind {
    fn from_char(c: char) -> Self {
        match c {
            'S' => TileKind::Start,
            '.' => TileKind::Ground,
            '|' => TileKind::Vertical,
            '-' => TileKind::Horizontal,
            'L' => TileKind::NorthEast,
            'J' => TileKind::NorthWest,
            'F' => TileKind::SouthEast,
            '7' => TileKind::SouthWest,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Tile {
    kind: TileKind,
    x: usize,
    y: usize,
}

impl Tile {
    fn new(c: char, x: usize, y: usize) -> Self {
        Self {
            kind: TileKind::from_char(c),
            x,
            y,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    fn new(input: &str) -> Self {
        let tiles = input
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| Tile::new(c, x, y))
                    .collect()
            })
            .collect();

        Self { tiles }
    }

    fn start_tile(&self) -> &Tile {
        self.tiles
            .iter()
            .flatten()
            .find(|tile| tile.kind == TileKind::Start)
            .unwrap()
    }

    fn start_direction(&self) -> Direction {
        let start_tile = self.start_tile();
        let start_x = start_tile.x;
        let start_y = start_tile.y;

        if start_x < self.tiles.len() - 1 {
            match self.tiles[start_x + 1][start_y].kind {
                TileKind::Vertical | TileKind::NorthEast | TileKind::NorthWest => {
                    return Direction::South
                }
                _ => unreachable!(),
            };
        }

        if start_x > 0 {
            match self.tiles[start_x - 1][start_y].kind {
                TileKind::Vertical | TileKind::SouthEast | TileKind::SouthWest => {
                    return Direction::North
                }
                _ => unreachable!(),
            };
        }

        if start_y > self.tiles[start_x].len() - 1 {
            match self.tiles[start_x][start_y + 1].kind {
                TileKind::Horizontal | TileKind::NorthWest | TileKind::SouthWest => {
                    return Direction::East
                }
                _ => unreachable!(),
            };
        }

        if start_y > 0 {
            match self.tiles[start_x][start_y - 1].kind {
                TileKind::Horizontal | TileKind::NorthEast | TileKind::SouthEast => {
                    return Direction::West
                }
                _ => unreachable!(),
            };
        }

        Direction::North
    }

    fn next_tile(&self, tile: &Tile, direction: Direction) -> (&Tile, Direction) {
        let current_x = tile.x;
        let current_y = tile.y;

        match direction {
            Direction::North => {
                let next_tile = &self.tiles[current_x - 1][current_y];
                let next_direction = match next_tile.kind {
                    TileKind::Vertical => direction,
                    TileKind::SouthEast => Direction::East,
                    TileKind::SouthWest => Direction::West,
                    TileKind::Start => Direction::North,
                    _ => unreachable!(),
                };

                (next_tile, next_direction)
            }
            Direction::South => {
                let next_tile = &self.tiles[current_x + 1][current_y];
                let next_direction = match next_tile.kind {
                    TileKind::Vertical => direction,
                    TileKind::NorthEast => Direction::East,
                    TileKind::NorthWest => Direction::West,
                    TileKind::Start => Direction::North,
                    _ => unreachable!(),
                };

                (next_tile, next_direction)
            }
            Direction::East => {
                let next_tile = &self.tiles[current_x][current_y + 1];
                let next_direction = match next_tile.kind {
                    TileKind::Horizontal => direction,
                    TileKind::NorthWest => Direction::North,
                    TileKind::SouthWest => Direction::South,
                    TileKind::Start => Direction::North,
                    _ => unreachable!(),
                };

                (next_tile, next_direction)
            }
            Direction::West => {
                let next_tile = &self.tiles[current_x][current_y - 1];
                let next_direction = match next_tile.kind {
                    TileKind::Horizontal => direction,
                    TileKind::NorthEast => Direction::North,
                    TileKind::SouthEast => Direction::South,
                    TileKind::Start => Direction::North,
                    _ => unreachable!(),
                };

                (next_tile, next_direction)
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    let tile_map = TileMap::new(input);
    let mut tile = tile_map.start_tile();
    let mut direction = tile_map.start_direction();
    let mut sum = 0;

    loop {
        (tile, direction) = tile_map.next_tile(tile, direction);
        sum += 1;

        if tile.kind == TileKind::Start {
            break;
        }
    }

    sum / 2
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input_1 = fs::read_to_string("input/test/day10_part_1_1.txt")?;
        let input_2 = fs::read_to_string("input/test/day10_part_1_2.txt")?;

        let result_1 = part_1(&input_1);
        let result_2 = part_1(&input_2);

        assert_eq!(result_1, 4);
        assert_eq!(result_2, 8);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input_1 = fs::read_to_string("input/test/day10_part_2_1.txt")?;
        let input_2 = fs::read_to_string("input/test/day10_part_2_2.txt")?;

        let result_1 = part_2(&input_1);
        let result_2 = part_2(&input_2);

        assert_eq!(result_1, 0);
        assert_eq!(result_2, 0);

        Ok(())
    }
}
