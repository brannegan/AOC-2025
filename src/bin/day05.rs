use anyhow::{Ok, Result};
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Cafeteria {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    available_ids: Vec<u64>,
}

fn parse(input: &str) -> Cafeteria {
    let fresh_ranges: Vec<RangeInclusive<u64>> = input
        .lines()
        .flat_map(|line| {
            line.split_once('-').map(|(start, end)| {
                start.parse().expect("parse start failed")..=end.parse().expect("parse end failed")
            })
        })
        .collect();
    let available_ids = input
        .lines()
        .skip(fresh_ranges.len() + 1)
        .map(|id| id.parse().expect("parse id"))
        .collect();
    Cafeteria {
        fresh_ranges,
        available_ids,
    }
}
fn fresh_from_available_ids(cafe: &Cafeteria) -> usize {
    cafe.available_ids
        .iter()
        .filter(|id| cafe.fresh_ranges.iter().any(|range| range.contains(id)))
        .count()
}
fn fresh_from_ranges(cafe: &mut Cafeteria) -> u64 {
    cafe.fresh_ranges
        .sort_by(|a, b| match a.start().cmp(b.start()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.end().cmp(b.end()),
        });
    let mut merge_ranges = vec![cafe.fresh_ranges[0].clone()];

    for next in cafe.fresh_ranges.iter().skip(1) {
        let prev = unsafe { merge_ranges.last().unwrap_unchecked() };
        if prev.end() < next.start() {
            merge_ranges.push(next.clone());
        } else if next.start() <= prev.end() && prev.end() <= next.end() {
            let prev = merge_ranges.pop().unwrap();
            merge_ranges.push(*prev.start()..=*next.end());
        } else {
            continue;
        }
    }
    merge_ranges.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day05-input1.txt")?;
    let mut cafe = parse(input.trim());
    let answer = fresh_from_available_ids(&cafe);
    println!("part 1 answer is: {answer}");
    let answer2 = fresh_from_ranges(&mut cafe);
    println!("part 2 answer is: {answer2}");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;
    #[test]
    fn part1() -> Result<()> {
        let cafe = parse(INPUT.trim());
        let answer = fresh_from_available_ids(&cafe);
        assert_eq!(answer, 3);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let mut cafe = parse(INPUT.trim());
        let answer = fresh_from_ranges(&mut cafe);
        assert_eq!(answer, 14);
        Ok(())
    }
}
