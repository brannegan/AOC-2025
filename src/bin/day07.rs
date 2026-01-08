use anyhow::{Ok, Result};
use glam::IVec2;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Map {
    start: IVec2,
    splitters: HashSet<IVec2>,
    beams: HashMap<IVec2, usize>,
    height: usize,
    splits: usize,
}

impl Map {
    fn beam_step_down(&mut self, beam: IVec2) {
        let timeline = *self.beams.get(&beam).expect("beam not found");
        if self.splitters.contains(&(beam + IVec2::new(0, 1))) {
            self.beams
                .entry(beam + IVec2::new(-1, 1))
                .and_modify(|tl| *tl += timeline)
                .or_insert(timeline);
            self.beams
                .entry(beam + IVec2::new(1, 1))
                .and_modify(|tl| *tl += timeline)
                .or_insert(timeline);
            self.splits += 1;
        } else {
            self.beams
                .entry(beam + IVec2::new(0, 1))
                .and_modify(|tl| *tl += timeline)
                .or_insert(timeline);
        }
    }
    fn run_beams(&mut self) {
        self.beams.insert(self.start, 1);
        self.beam_step_down(self.start);
        for row in 1..self.height {
            self.beams.retain(|beam,_| beam.y == row as i32);
            let beams: Vec<_> = self.beams.keys().cloned().collect();
            beams.into_iter().for_each(|beam| self.beam_step_down(beam));
        }
    }
    fn beam_timelines(&self) -> usize {
        self.beams
            .iter()
            .filter(|(beam, _)| beam.y == self.height as i32)
            .map(|(_, tl)| tl)
            .sum()
    }
}

fn parse(input: &str) -> Map {
    let start = IVec2 {
        x: input.find('S').unwrap() as i32,
        y: 0,
    };
    let height = input.lines().count();
    let splitters = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_col, ch)| *ch == '^')
                .map(move |(col, _ch)| IVec2::new(col as i32, row as i32))
        })
        .collect();

    Map {
        start,
        splitters,
        beams: HashMap::new(),
        height,
        splits: 0,
    }
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day07-input1.txt")?;
    let mut map = parse(input.trim());
    map.run_beams();
    let answer = map.splits;
    println!("part 1 answer is: {answer}");
    let answer = map.beam_timelines();
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn part1() -> Result<()> {
        let mut map = parse(INPUT.trim());
        map.run_beams();
        let answer = map.splits;
        assert_eq!(answer, 21);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let mut map = parse(INPUT.trim());
        map.run_beams();
        let answer = map.beam_timelines();
        assert_eq!(answer, 40);
        Ok(())
    }
}
