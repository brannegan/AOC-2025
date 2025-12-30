use anyhow::{Ok, Result};
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Bank(Vec<i64>);

impl Bank {
    fn max_jolt2(&self) -> i64 {
        let bank_iter = self.0.iter().map(|&j| -j);
        let max_pos1 = bank_iter
            .clone()
            .position_min()
            .expect("first max not found");
        let max_pos2 = if max_pos1 == self.0.len() - 1 {
            bank_iter
                .take(max_pos1)
                .position_min()
                .expect("second max not found")
        } else {
            bank_iter
                .skip(max_pos1 + 1)
                .position_min()
                .expect("second max not found")
                + max_pos1
                + 1
        };
        if max_pos1 > max_pos2 {
            self.0[max_pos2] * 10 + self.0[max_pos1]
        } else {
            self.0[max_pos1] * 10 + self.0[max_pos2]
        }
    }
    fn max_jolt12(&self) -> i64 {
        const MAX_BATTARIES: usize = 12;
        let mut max_jolt_idxs = Vec::with_capacity(MAX_BATTARIES);
        let mut cur_slice = &self.0[..];
        while max_jolt_idxs.len() < MAX_BATTARIES {
            let next_start_pos = max_jolt_idxs.last().copied().map(|l| l + 1).unwrap_or(0);
            let max_pos = max_in_slice(cur_slice);
            if next_start_pos + max_pos + MAX_BATTARIES > self.0.len() + max_jolt_idxs.len() {
                cur_slice = &cur_slice[..max_pos];
            } else {
                max_jolt_idxs.push(next_start_pos + max_pos);
                cur_slice = &self.0[next_start_pos + max_pos + 1..];
            }
        }
        max_jolt_idxs
            .iter()
            .sorted()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, jolt)| {
                acc + self.0[*jolt] * 10_i64.pow(i as u32)
            })
    }
}
fn max_in_slice(slice: &[i64]) -> usize {
    slice
        .iter()
        .map(|j| -j)
        .position_min()
        .expect("empty slice")
}

fn parse(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|line| {
            Bank(
                line.chars()
                    .map(|c| c.to_digit(10).expect("digit") as i64)
                    .collect(),
            )
        })
        .collect()
}
fn main() -> Result<()> {
    let input = read_to_string("inputs/day03-input1.txt")?;
    let banks = parse(input.trim());
    let answer = banks.iter().map(|bank| bank.max_jolt2()).sum::<i64>();
    println!("part 1 answer is: {answer}");
    let answer = banks.iter().map(|bank| bank.max_jolt12()).sum::<i64>();
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;
    #[test]
    fn part1() -> Result<()> {
        let answer = parse(INPUT.trim())
            .iter()
            .map(|bank| bank.max_jolt2())
            .sum::<i64>();
        assert_eq!(answer, 357);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let answer = parse(INPUT.trim())
            .iter()
            .map(|bank| bank.max_jolt12())
            .inspect(|j| {
                eprintln!("j = {:#?}", j);
            })
            .sum::<i64>();
        assert_eq!(answer, 3121910778619);
        Ok(())
    }
}
