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
    distance: u16,
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
    let answer = count_dial_is_zero(input.trim(), 50)?;
    println!("answer is: {answer}");
    Ok(())
}

fn count_dial_is_zero(input: &str, start_pos: i16) -> Result<u32> {
    let (_last_pos, zeros) = input.lines().map(Rotation::from_str).try_fold(
        (start_pos, 0),
        |(cur_pos, zeros), rot_parsed| {
            let rot = rot_parsed?;
            let new_pos = match rot.dir {
                Dir::Left => (cur_pos - rot.distance as i16) % 100,
                Dir::Right => (cur_pos + rot.distance as i16) % 100,
            };
            Ok((new_pos, if new_pos == 0 { zeros + 1 } else { zeros }))
        },
    )?;
    Ok(zeros)
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
    fn task_input() -> Result<()> {
        assert_eq!(count_dial_is_zero(INPUT.trim(), 50)?, 3);
        Ok(())
    }
}
