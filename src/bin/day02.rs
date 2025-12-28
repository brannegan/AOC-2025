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
fn is_invalid_id_part1(id: &u64) -> bool {
    let id = id.to_string();
    id.len().is_multiple_of(2) && id[..id.len() / 2] == id[id.len() / 2..]
}
fn is_invalid_id_part2(id: &u64) -> bool {
    let id_str = id.to_string();
    let double_id_str = id_str.repeat(2);
    double_id_str[1..double_id_str.len()-1].contains(&id_str)
}

fn invalid_ids_sum(ranges: &[RangeInclusive<u64>], pred: fn(&u64) -> bool) -> u64 {
    ranges
        .iter()
        .flat_map(|range| {
            range.clone().filter(pred)
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day02-input1.txt")?;
    let ranges = parse(input.trim())?;
    let answer = invalid_ids_sum(&ranges, is_invalid_id_part1);
    println!("part 1 answer is: {answer}");
    let answer2 = invalid_ids_sum(&ranges, is_invalid_id_part2);
    println!("part 2 answer is: {answer2}");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#;
    #[test]
    fn part1() -> Result<()> {
        let ranges = parse(INPUT.trim())?;
        assert_eq!(invalid_ids_sum(&ranges, is_invalid_id_part1), 1227775554);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let ranges = parse(INPUT.trim())?;
        assert_eq!(invalid_ids_sum(&ranges, is_invalid_id_part2), 4174379265);
        Ok(())
    }
}
