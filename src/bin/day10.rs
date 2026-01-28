use anyhow::{Ok, Result};
use glam::bool;
use lpsolve::Problem;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space1, usize};
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::{Finish, Parser};
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Button(Vec<usize>);

impl Button {
    fn new(items: Vec<usize>) -> Self {
        Self(items)
    }
    fn press(&self, lights_state: &mut [bool]) {
        for i in self.0.iter().copied() {
            lights_state[i] = !lights_state[i];
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    lights_diagram: Vec<bool>,
    buttons: Vec<Button>,
    joltage_req: Vec<usize>,
    lights_state: Vec<bool>,
}

impl Machine {
    fn new(light_diagram: Vec<bool>, buttons: Vec<Button>, joltage_req: Vec<usize>) -> Self {
        let len = light_diagram.len();
        Self {
            lights_diagram: light_diagram,
            buttons,
            joltage_req,
            lights_state: vec![false; len],
        }
    }
    fn lights_reset(&mut self) {
        self.lights_state
            .iter_mut()
            .for_each(|light| *light = false);
    }
    fn has_light_state_cycle(&mut self, btn_history: &[usize]) -> bool {
        let mut light_state = HashSet::new();
        for b in btn_history.iter() {
            self.buttons[*b].press(&mut self.lights_state);
            if !light_state.insert(self.lights_state.clone()) {
                return true;
            }
        }
        false
    }
    fn min_buttons_seq(&mut self) -> usize {
        let mut min = usize::MAX;
        let mut btn_queue = VecDeque::from_iter((0..self.buttons.len()).map(|btn_i| vec![btn_i]));
        while let Some(btn_history) = btn_queue.pop_front() {
            self.lights_reset();
            if btn_history.len() > min || self.has_light_state_cycle(&btn_history) {
                continue;
            } else if self.lights_state == self.lights_diagram {
                min = btn_history.len().min(min);
                continue;
            } else {
                let last_btn_pressed = btn_history.last().unwrap();
                for btn_i in (0..self.buttons.len()).filter(|btn_i| btn_i != last_btn_pressed) {
                    let mut next = btn_history.clone();
                    next.push(btn_i);
                    btn_queue.push_back(next);
                }
            }
        }
        min
    }
    fn min_buttons_joltage(&self) -> usize {
        let cols = self.buttons.len();
        let mut problem = Problem::builder()
            .cols(cols as i32)
            .min(&vec![1.; cols])
            .integer_vars(&(1..=cols as i32).collect::<Vec<_>>())
            .non_negative_integers()
            .verbosity(lpsolve::Verbosity::Critical);
        for j in 0..self.joltage_req.len() {
            let coeffs: Vec<_> = self
                .buttons
                .iter()
                .map(|btn| if btn.0.contains(&j) { 1. } else { 0. }).collect();
            problem = problem.eq(&coeffs, self.joltage_req[j] as f64);
        }
        let solution = problem.solve().expect("unable to solve lp problem");
        solution.objective_value().ceil() as usize
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Machine>> {
    let light = one_of(".#").map(|c| match c {
        '.' => false,
        '#' => true,
        _ => unimplemented!(),
    });
    let light_diagram = delimited(tag("["), many1(light), tag("]"));
    let button = delimited(tag("("), separated_list1(tag(","), usize), tag(")")).map(Button::new);
    let joltage_req = delimited(tag("{"), separated_list1(tag(","), usize), tag("}"));
    let machine = (
        light_diagram,
        space1,
        separated_list0(space1, button),
        space1,
        joltage_req,
    )
        .map(|(light_diagram, _, buttons, _, joltage_req)| {
            Machine::new(light_diagram, buttons, joltage_req)
        });
    let mut parser = separated_list1(line_ending, machine);
    let (_rest, machines) = parser
        .parse_complete(input)
        .finish()
        .map_err(|e: nom::error::Error<&str>| anyhow::anyhow!("parser error: {:?}", e))?;
    Ok(machines)
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day10-input1.txt")?;
    let mut machines = parse(input.trim())?;
    let answer = machines
        .iter_mut()
        .map(|machine| machine.min_buttons_seq())
        .sum::<usize>();
    println!("part 1 answer is: {answer}");
    let answer = machines
        .iter()
        .map(|machine| machine.min_buttons_joltage())
        .sum::<usize>();
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;
    #[test]
    fn part1() -> Result<()> {
        let mut machines = parse(INPUT.trim())?;
        let answer = machines
            .iter_mut()
            .map(|machine| machine.min_buttons_seq())
            .sum::<usize>();
        assert_eq!(answer, 7);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let mut machines = parse(INPUT.trim())?;
        let answer = machines
            .iter_mut()
            .map(|machine| machine.min_buttons_joltage())
            .sum::<usize>();
        assert_eq!(answer, 33);
        Ok(())
    }
}
