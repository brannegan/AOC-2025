use anyhow::{Ok, Result};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Mul,
}
#[derive(Debug, Clone)]
struct Homework {
    numbers: Vec<Vec<u64>>,
    ops: Vec<Op>,
}

impl Homework {
    fn solve(&self) -> u64 {
        let mut result = 0;
        for i in 0..self.ops.len() {
            result += match self.ops[i] {
                Op::Plus => self.numbers.iter().fold(0, |acc, col| acc + col[i]),
                Op::Mul => self.numbers.iter().fold(1, |acc, col| acc * col[i]),
            }
        }
        result
    }
}
fn parse_col(lines: &[&[u8]], i: usize) -> u64 {
    let mut num_as_str = String::new();
    for line in lines {
        if line[i] == b' ' {
            continue;
        }
        num_as_str.push(line[i].into());
    }
    num_as_str.parse::<u64>().unwrap()
}

fn solve2(input: &str) -> u64 {
    let len = input.lines().count();
    let ops = input
        .lines()
        .nth(len - 1)
        .unwrap()
        .match_indices(['*', '+'])
        .map(|(i, c)| {
            (
                i,
                match c {
                    "*" => Op::Mul,
                    "+" => Op::Plus,
                    _ => unreachable!("no other operations"),
                },
            )
        })
        .collect::<Vec<_>>();
    let lines: Vec<_> = input.lines().take(len - 1).map(|s| s.as_bytes()).collect();
    let mut col_end = lines[0].len();
    let mut result = 0;
    for (col_start, op) in ops.iter().rev() {
        result += match op {
            Op::Plus => (*col_start..col_end)
                .rev()
                .fold(0, |acc, i| acc + parse_col(&lines, i)),
            Op::Mul => (*col_start..col_end)
                .rev()
                .fold(1, |acc, i| acc * parse_col(&lines, i)),
        };
        col_end = col_start.saturating_sub(1);
    }
    result
}
fn parse(input: &str) -> Homework {
    let numbers: Vec<_> = input
        .lines()
        .filter(|line| line.starts_with(|c: char| c != '*' && c != '+'))
        .map(|line| {
            line.split_whitespace()
                .map(|num_as_str| num_as_str.parse().unwrap())
                .collect()
        })
        .collect();
    let ops = input
        .lines()
        .skip(numbers.len())
        .map(|line| {
            line.split_whitespace()
                .map(|c| match c {
                    "*" => Op::Mul,
                    "+" => Op::Plus,
                    _ => unreachable!("no other operations"),
                })
                .collect()
        })
        .next()
        .unwrap();
    Homework { numbers, ops }
}
fn main() -> Result<()> {
    let input = read_to_string("inputs/day06-input1.txt")?;
    let homework = parse(input.trim());
    let answer = homework.solve();
    println!("part 1 answer is: {answer}");
    let answer = solve2(input.trim());
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + 
"#;
    #[test]
    fn part1() -> Result<()> {
        let homework = parse(INPUT.trim());
        let answer = homework.solve();
        assert_eq!(answer, 4277556);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let answer = solve2(INPUT.trim());
        assert_eq!(answer, 3263827);
        Ok(())
    }
}
