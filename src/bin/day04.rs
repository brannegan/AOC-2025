use anyhow::{Ok, Result};
use glam::IVec2;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Map(HashSet<IVec2>);

fn parse(input: &str) -> Map {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_col, ch)| *ch == '@')
                .map(move |(col, _ch)| IVec2::new(row as i32, col as i32))
        })
        .collect();

    Map(map)
}
fn rolls_can_be_accessed(map: &Map) -> impl Iterator<Item = IVec2> {
    map.0.iter().filter_map(|roll| {
        let adj_rolls = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter(|&adj| map.0.contains(&(roll + IVec2::from(*adj))))
        .count();
        if adj_rolls < 4 { Some(*roll) } else { None }
    })
}
fn rolls_can_be_removed(map: &mut Map) -> usize {
    let rolls_to_remove = rolls_can_be_accessed(map).collect::<Vec<_>>();
    if rolls_to_remove.is_empty() {
        return 0;
    }
    for roll in &rolls_to_remove {
        map.0.remove(roll);
    }
    rolls_to_remove.len() + rolls_can_be_removed(map)
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day04-input1.txt")?;
    let mut map = parse(input.trim());
    let answer = rolls_can_be_accessed(&map).count();
    println!("part 1 answer is: {answer}");
    let answer = rolls_can_be_removed(&mut map);
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;
    #[test]
    fn part1() -> Result<()> {
        let map = parse(INPUT.trim());
        let answer = rolls_can_be_accessed(&map).count();
        assert_eq!(answer, 13);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let mut map = parse(INPUT.trim());
        let answer = rolls_can_be_removed(&mut map);
        assert_eq!(answer, 43);
        Ok(())
    }
}
