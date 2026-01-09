use anyhow::{Ok, Result};
use glam::I64Vec2;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct RedTileList {
    coords: Vec<I64Vec2>,
}

impl RedTileList {
    fn new(coords: Vec<I64Vec2>) -> Self {
        Self { coords }
    }
    fn max_area(&self) -> i64 {
        let mut max_area = 0;
        let len = self.coords.len();
        for i in 0..len {
            for j in i + 1..len {
                let area = ((self.coords[i].x - self.coords[j].x).abs() + 1)
                    * ((self.coords[i].y - self.coords[j].y).abs() + 1);
                max_area = max_area.max(area);
            }
        }
        max_area
    }
}

fn parse(input: &str) -> RedTileList {
    let coords = input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| I64Vec2::from((x.parse().unwrap(), y.parse().unwrap())))
                .unwrap()
        })
        .collect();
    RedTileList::new(coords)
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day09-input1.txt")?;
    let red_tiles = parse(input.trim());
    let answer = red_tiles.max_area();
    println!("part 1 answer is: {answer}");

    let red_tiles = parse(input.trim());
    let answer = red_tiles.max_area();
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;
    #[test]
    fn part1() -> Result<()> {
        let red_tiles = parse(INPUT.trim());
        let answer = red_tiles.max_area();
        assert_eq!(answer, 50);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let red_tiles = parse(INPUT.trim());
        let answer = red_tiles.max_area();
        assert_eq!(answer, 24);
        Ok(())
    }
}
