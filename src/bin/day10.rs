use anyhow::{Ok, Result};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space1, usize};
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::{Finish, Parser};
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Button(Vec<usize>);

impl Button {
    fn new(items: Vec<usize>) -> Self {
        Self(items)
    }
    fn press(&self, mut lights_state: Vec<bool>) -> Vec<bool> {
        for i in self.0.iter().copied() {
            lights_state[i] = !lights_state[i];
        }
        lights_state
    }
}

#[derive(Debug, Clone)]
struct Machine {
    lights_diagram: Vec<bool>,
    buttons: Vec<Button>,
    _joltage_req: Vec<usize>,
    lights_state: Vec<bool>,
}

impl Machine {
    fn new(light_diagram: Vec<bool>, buttons: Vec<Button>, joltage_req: Vec<usize>) -> Self {
        let len = light_diagram.len();
        Self {
            lights_diagram: light_diagram,
            buttons,
            _joltage_req: joltage_req,
            lights_state: vec![false; len],
        }
    }
    fn next_button(
        &self,
        button_i: usize,
        lights_state: Vec<bool>,
        mut btn_states: HashSet<Vec<bool>>,
        mut count: usize,
    ) -> Option<(HashSet<Vec<bool>>, usize)> {
        let new_state = self.buttons[button_i].press(lights_state.clone());
        count += 1;
        if new_state == self.lights_diagram {
            return Some((btn_states, count));
        }
        if !btn_states.insert(new_state.clone()) {
            return None;
        }
        (0..self.buttons.len())
            .filter(|&btn_i| btn_i != button_i)
            .filter_map(|btn_i| {
                self.next_button(btn_i, new_state.clone(), btn_states.clone(), count)
            })
            .min_by_key(|(_, count)| *count)
    }
    fn min_buttons_seq(&self) -> usize {
        let mut state = HashSet::new();
        state.insert(self.lights_state.clone());
        (0..self.buttons.len())
            .filter_map(|button_i| {
                self.next_button(button_i, self.lights_state.clone(), state.clone(), 0)
            })
    //        .inspect(|(bs, c)| eprintln!("button_states {bs:?}, count={c}"))
            .map(|(_, count)| count)
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
    let input = read_to_string("inputs/day10-input1.txt")?;
    let machines = parse(input.trim())?;

    let answer = machines
        .iter()
        .map(|machine| machine.min_buttons_seq())
        .sum::<usize>();
    println!("part 1 answer is: {answer}");
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
        let machines = parse(INPUT.trim())?;
        let answer = machines
            .iter()
            .map(|machine| machine.min_buttons_seq())
            .sum::<usize>();
        assert_eq!(answer, 7);
        Ok(())
    }
    #[test]
    fn min_buttons_seq() -> Result<()> {
        let machines = parse(INPUT.trim())?;
        assert_eq!(machines[0].min_buttons_seq(), 2);
        assert_eq!(machines[1].min_buttons_seq(), 3);
        assert_eq!(machines[2].min_buttons_seq(), 2);
        Ok(())
    }
    // #[test]
    // fn part2() -> Result<()> {
    //     let mut machines = parse(INPUT.trim());
    //     let answer = 0;
    //     assert_eq!(answer, 24);
    //     Ok(())
    // }
}
