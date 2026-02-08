#![allow(dead_code)]
use anyhow::{Ok, Result};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, line_ending, one_of, space1, usize};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::{Finish, Parser};
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Shape([[u8; 3]; 3]);

impl Shape {
    fn new(arr: [u8; 9]) -> Self {
        Self([
            [arr[0], arr[1], arr[2]],
            [arr[3], arr[4], arr[5]],
            [arr[6], arr[7], arr[8]],
        ])
    }
    fn rot_left(&self) -> Self {
        let mat = self.0;
        let left = [
            [mat[0][2], mat[1][2], mat[2][2]],
            [mat[0][1], mat[1][1], mat[2][1]],
            [mat[0][0], mat[1][0], mat[2][0]],
        ];
        Self(left)
    }
    fn rot_right(&self) -> Self {
        self.rot_left().rot_left().rot_left()
    }
    fn rot_180(&self) -> Self {
        self.rot_left().rot_left()
    }
    fn flip_vert(&self) -> Self {
        let mat = self.0;
        let flip_vert = [
            [mat[0][2], mat[0][1], mat[0][0]],
            [mat[1][2], mat[1][1], mat[1][0]],
            [mat[2][2], mat[2][1], mat[2][0]],
        ];
        Self(flip_vert)
    }
    fn flip_hor(&self) -> Self {
        let mat = self.0;
        let flip_hor = [mat[2], mat[1], mat[0]];
        Self(flip_hor)
    }
}

#[derive(Debug, Clone, Hash)]
struct Region {
    shape_ids: [usize; 6],
    state: Vec<Vec<u8>>,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.state.len() {
            for j in 0..self.state[0].len() {
                write!(f, "{}", if self.state[i][j] > 0 { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        Result::Ok(())
    }
}
impl Region {
    fn new(shape_ids: Vec<usize>, w: usize, h: usize) -> Self {
        let shape_ids = shape_ids
            .into_iter()
            .collect_array::<6>()
            .expect("parse region failed");
        Self {
            shape_ids,
            state: vec![vec![0; w]; h],
        }
    }
    fn add_shape(mut self, shape: Shape, r: usize, c: usize) -> Option<Self> {
        for i in 0..3 {
            for j in 0..3 {
                self.state[r + i][c + j] += shape.0[i][j];
                if self.state[r + i][c + j] == 2 {
                    return None;
                }
            }
        }
        Some(self)
    }

    fn shapes_fit(&self, shapes: &[Shape]) -> bool {
        let mut queue = vec![self.clone()];
        let w = self.state[0].len();
        let h = self.state.len();

        while let Some(mut cur) = queue.pop() {
            let shape_ids = cur.shape_ids;
            if shape_ids.iter().all(|count| *count == 0) {
                eprintln!("{cur}");
                return true;
            }
            let shape_id = shape_ids.iter().position(|count| *count > 0).unwrap();
            cur.shape_ids[shape_id] -= 1;
            let new_shapes = HashSet::from([
                shapes[shape_id],
                shapes[shape_id].rot_left(),
                shapes[shape_id].rot_right(),
                shapes[shape_id].rot_180(),
                shapes[shape_id].flip_hor(),
                shapes[shape_id].flip_vert(),
            ]);
            queue.extend(new_shapes.iter().flat_map(|shape| {
                (0..=h - 3)
                    .cartesian_product(0..=w - 3)
                    .filter(|(r, c)| cur.state[*r][*c] == 0)
                    .filter_map(|(r, c)| cur.clone().add_shape(*shape, r, c))
            }));
        }
        false
    }
}
#[derive(Debug, Clone)]
struct Tetris {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Tetris {
    fn stats(&self) {
        let shapes_occupied_space: Vec<_> = self
            .shapes
            .iter()
            .map(|shape| shape.0.iter().flatten().filter(|h| **h == 1).count())
            .collect();
        for region in &self.regions {
            let occupied_total: usize = region
                .shape_ids
                .iter()
                .enumerate()
                .map(|(i, c)| shapes_occupied_space[i] * c)
                .sum();
            println!(
                "{}x{} {:?}: occupied {occupied_total} vs total {}",
                region.state[0].len(),
                region.state.len(),
                region.shape_ids,
                region.state[0].len() * region.state.len()
            );
        }
    }
    fn fit_heuristic(&self) -> usize {
        let shapes_occupied_space: Vec<_> = self
            .shapes
            .iter()
            .map(|shape| shape.0.iter().flatten().filter(|h| **h == 1).count())
            .collect();
        self.regions
            .iter()
            .filter(|region| {
                region
                    .shape_ids
                    .iter()
                    .enumerate()
                    .map(|(i, c)| shapes_occupied_space[i] * c)
                    .sum::<usize>()
                    < region.state[0].len() * region.state.len()
            })
            .count()
    }
}

fn parse(input: &str) -> anyhow::Result<Tetris> {
    let shape_line = many1(one_of(".#").map(|c| match c {
        '.' => 0_u8,
        '#' => 1,
        _ => unimplemented!(),
    }));
    let shape = delimited(
        i32.and(tag(":").and(line_ending)),
        separated_list1(line_ending, shape_line),
        line_ending,
    )
    .map(|shape| {
        let arr = shape
            .iter()
            .flatten()
            .copied()
            .collect_array::<9>()
            .expect("parse shape failed");
        Shape::new(arr)
    });
    let shapes = separated_list1(line_ending, shape);
    let dims = separated_pair(usize, tag("x"), usize);
    let shape_ids = separated_list1(space1, usize);
    let region =
        separated_pair(dims, tag(": "), shape_ids).map(|((w, h), ids)| Region::new(ids, w, h));
    let regions = separated_list1(line_ending, region);
    let mut parser = separated_pair(shapes, line_ending, regions);
    let (_rest, (shapes, regions)) = parser
        .parse_complete(input)
        .finish()
        .map_err(|e: nom::error::Error<&str>| anyhow::anyhow!("parser error: {:?}", e))?;
    Ok(Tetris { shapes, regions })
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day12-input1.txt")?;
    let tree_farm = parse(input.trim())?;
    let answer = tree_farm.fit_heuristic();
    println!("part 1 answer is: {answer}");
    // let answer = 0
    // println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = r#"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;
    #[test]
    fn part1() -> Result<()> {
        let tree_farm = parse(INPUT.trim())?;
        dbg!(tree_farm);
        let answer = 0;
        assert_eq!(answer, 7);
        Ok(())
    }
    #[test]
    fn shapes_fit() -> Result<()> {
        let tree_farm = parse(INPUT.trim())?;
        let region0 = tree_farm.regions[0].clone();
        assert!(region0.shapes_fit(&tree_farm.shapes));
        let region1 = tree_farm.regions[1].clone();
        assert!(region1.shapes_fit(&tree_farm.shapes));
        let region2 = tree_farm.regions[2].clone();
        assert!(!region2.shapes_fit(&tree_farm.shapes));
        Ok(())
    }
}
