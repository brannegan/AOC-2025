use anyhow::{Error, Ok, Result, anyhow};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Dir::Left),
            'R' => Ok(Dir::Right),
            _ => Err(anyhow!("unknown direction")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    dir: Dir,
    distance: i16,
}

impl Rotation {
    fn count_zero_passes(&self, pos: i16) -> (i16, i16) {
        let full_circles = (self.distance / 100).abs();
        let dist_rem = self.distance % 100;
        let clockwise_sign = match self.dir {
            Dir::Left => -1,
            Dir::Right => 1,
        };
        let next_pos = pos + dist_rem * clockwise_sign;
        let zero_pass = if (next_pos > 0 && next_pos < 100) || pos == 0 {
            0
        } else {
            1
        };
        ((100 + next_pos) % 100, full_circles + zero_pass)
    }
}

impl FromStr for Rotation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut line = s.chars();
        let dir = line.next().ok_or(anyhow!("empty line"))?.try_into()?;
        let distance = line.as_str().parse()?;
        Ok(Rotation { dir, distance })
    }
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day01-input1.txt")?;
    let answer = count_dial_finished_zero(input.trim(), 50)?;
    println!("part 1 answer is: {answer}");
    let answer = count_dials_crosses_zero(input.trim(), 50)?;
    println!("part 2 answer is: {answer}");
    Ok(())
}

fn count_dial_finished_zero(input: &str, start_pos: i16) -> Result<u32> {
    let (_last_pos, zeros) = input.lines().map(Rotation::from_str).try_fold(
        (start_pos, 0),
        |(cur_pos, zeros), rot_parsed| {
            let rot = rot_parsed?;
            let new_pos = match rot.dir {
                Dir::Left => (cur_pos - rot.distance + 100) % 100,
                Dir::Right => (cur_pos + rot.distance) % 100,
            };
            Ok((new_pos, if new_pos == 0 { zeros + 1 } else { zeros }))
        },
    )?;
    Ok(zeros)
}

fn count_dials_crosses_zero(input: &str, start_pos: i16) -> Result<i16> {
    Ok(input.lines().map(Rotation::from_str).try_fold(
        (start_pos, 0),
        |(cur_pos, zeros), rot_parsed| {
            let (next_pos, cur_zeros) = rot_parsed?.count_zero_passes(cur_pos);
            Ok((next_pos, zeros + cur_zeros))
        },
    )?.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;
    #[test]
    fn part1() -> Result<()> {
        assert_eq!(count_dial_finished_zero(INPUT.trim(), 50)?, 3);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        assert_eq!(count_dials_crosses_zero(INPUT.trim(), 50)?, 6);
        Ok(())
    }
}
