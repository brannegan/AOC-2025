use anyhow::{Ok, Result};
use glam::bool;
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
    fn press_counters(&self, mut counters: Vec<usize>) -> Option<(Vec<usize>, usize)> {
        if self.0.iter().any(|i| counters[*i] == 0) {
            return None;
        }
        let min_pressed = self
            .0
            .iter()
            .copied()
            .map(|btn_i| counters[btn_i])
            .min()
            .expect("min not found");
        for i in self.0.iter().copied() {
            counters[i] -= min_pressed;
        }
        Some((counters, min_pressed))
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
    fn try_next_button(
        &self,
        button_i: usize,
        counters: Vec<usize>,
        pressed_so_far: usize,
    ) -> Option<(Vec<usize>, usize)> {
        let (new_counters, pressed_now) = self.buttons[button_i].press_counters(counters)?;
        //  eprintln!(
        //      "btn={button_i}, counters={:?}, {pressed_so_far}, {pressed_now}",
        //      new_counters
        // );
        if new_counters == vec![0; self.joltage_req.len()] {
            return Some((new_counters, pressed_so_far + pressed_now));
        }
        (0..self.buttons.len())
            .filter(|&btn_i| btn_i != button_i)
            .filter_map(|btn_i| {
                self.try_next_button(btn_i, new_counters.clone(), pressed_so_far + pressed_now)
            })
            .min_by_key(|(_, count)| *count)
    }
    fn min_buttons_joltage(&self) -> usize {
        (0..self.buttons.len())
            .filter_map(|button_i| self.try_next_button(button_i, self.joltage_req.clone(), 0))
            //.inspect(|(counters, pressed)| eprintln!("counters {counters:?}, pressed={pressed}"))
            .map(|(_, pressed)| pressed)
            .min()
            .unwrap()
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
    //let input = read_to_string("inputs/day10-input1.txt")?;
    //let mut machines = parse(input.trim())?;
    //let answer = machines
    //    .iter_mut()
    //    .map(|machine| machine.min_buttons_seq())
    //    //.inspect(|min| eprintln!("min={min}"))
    //    .sum::<usize>();
    //println!("part 1 answer is: {answer}");
    let input = read_to_string("inputs/day10-input1.txt")?;
    let machines = parse(input.trim())?;
    let answer = machines
        .iter()
        .map(|machine| machine.min_buttons_joltage())
        //.inspect(|min| eprintln!("min={min}"))
        .sum::<usize>();
    println!("part 2 answer is: {answer}");
    // println!("part 2 answer is: {answer}");
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
    #[test]
    fn part2_min_buttons_seq() -> Result<()> {
        let machines = parse(INPUT.trim())?;
        assert_eq!(machines[0].min_buttons_joltage(), 10);
        assert_eq!(machines[1].min_buttons_joltage(), 12);
        assert_eq!(machines[2].min_buttons_joltage(), 11);
        Ok(())
    }
}
