use std::fs::read_to_string;
use std::ops::RangeInclusive;

use anyhow::{Ok, Result, anyhow};

fn parse(input: &str) -> Result<Vec<RangeInclusive<u64>>> {
    input
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .map(|(start, end)| start.parse().unwrap()..=end.parse().unwrap())
                .ok_or(anyhow!("parse failed"))
        })
        .collect()
}

fn invalid_pattern_part1(range: &RangeInclusive<u64>) -> impl Iterator<Item=u64> {
    range
        .clone()
        .filter(|num| {
            let num_as_str = num.to_string();
            num_as_str.len() % 2 == 0 &&
            num_as_str[..num_as_str.len() / 2] == num_as_str[num_as_str.len() / 2..]
        })
}
fn invalid_ids_sum<'a,F,R>(ranges: &'a[RangeInclusive<u64>], func: F) -> u64
where
    R: Iterator<Item = u64>,
    F: Fn(&'a RangeInclusive<u64>) -> R,
{
    ranges.iter().flat_map(func).sum()
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day02-input1.txt")?;
    let ranges = parse(input.trim())?;
    let answer = invalid_ids_sum(&ranges,invalid_pattern_part1);
    println!("part 1 answer is: {answer}");
    //let answer = count_dials_crosses_zero(input.trim(), 50)?;
    //println!("part 2 answer is: {answer}");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#;
    #[test]
    fn part1_invalid_11_22() -> Result<()> {
        let ranges = parse(INPUT.trim())?;
        assert_eq!(invalid_pattern_part1(&ranges[0]).sum::<u64>(), 11 + 22);
        Ok(())
    }
    #[test]
    fn part1() -> Result<()> {
        let ranges = parse(INPUT.trim())?;
        assert_eq!(invalid_ids_sum(&ranges,invalid_pattern_part1), 1227775554);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        assert_eq!(6, 6);
        Ok(())
    }
}
