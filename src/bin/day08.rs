use anyhow::{Ok, Result};
use glam::I64Vec3;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct JBoxes {
    jboxes: Vec<I64Vec3>,
    circuits: Vec<HashSet<usize>>,
    last_conn: [I64Vec3; 2],
}

impl JBoxes {
    fn min_distances(&self) -> impl Iterator<Item = [usize; 2]> + use<> {
        let mut result = vec![];
        for i in 0..self.jboxes.len() {
            for j in i + 1..self.jboxes.len() {
                let dist = self.jboxes[i].distance_squared(self.jboxes[j]);
                result.push(([i, j], dist));
            }
        }
        result
            .into_iter()
            .sorted_by_key(|(_, dist)| *dist)
            .map(|(jbox_ids, _)| jbox_ids)
    }
    fn construct_circuits(&mut self, min_dists_iter: impl Iterator<Item = [usize; 2]>) {
        for conn in min_dists_iter {
            let h = HashSet::from(conn);
            let intersect_circuit_ids: Vec<usize> = self
                .circuits
                .iter()
                .enumerate()
                .filter(|(_, hm)| !hm.is_disjoint(&h))
                .map(|(i, _)| i)
                .collect();
            if intersect_circuit_ids.is_empty() {
                // add new circuit
                self.circuits.push(h);
            } else if intersect_circuit_ids.len() == 1 {
                // extend existing circuit
                self.circuits[intersect_circuit_ids[0]].extend(h);
            } else {
                // join existing circuits
                let removed = self.circuits.remove(intersect_circuit_ids[1]);
                self.circuits[intersect_circuit_ids[0]].extend(removed);
            }
            self.last_conn = [self.jboxes[conn[0]], self.jboxes[conn[1]]];
            if self.circuits[0].len() == self.jboxes.len() {
                // full circuit
                break;
            }
        }
    }
}

fn parse(input: &str) -> JBoxes {
    let jboxes = input
        .lines()
        .map(|line| {
            let arr = line
                .splitn(3, ',')
                .map(|coord| coord.parse::<i64>().unwrap())
                .collect_array()
                .expect("parse line failed");
            I64Vec3::from_array(arr)
        })
        .collect();
    JBoxes {
        jboxes,
        circuits: vec![],
        last_conn: [I64Vec3::ZERO; 2],
    }
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day08-input1.txt")?;
    let mut jboxes = parse(input.trim());
    jboxes.construct_circuits(jboxes.min_distances().take(1000));
    let answer = jboxes
        .circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted_by_key(|&len| -(len as isize))
        .take(3)
        .product::<usize>();
    println!("part 1 answer is: {answer}");

    jboxes.construct_circuits(jboxes.min_distances().skip(1000));
    let answer = jboxes.last_conn[0].x * jboxes.last_conn[1].x;
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;
    #[test]
    fn part1() -> Result<()> {
        let mut jboxes = parse(INPUT.trim());
        jboxes.construct_circuits(jboxes.min_distances().take(10));
        let answer = jboxes
            .circuits
            .iter()
            .map(|circuit| circuit.len())
            .sorted_by_key(|&len| -(len as isize))
            .take(3)
            .product::<usize>();
        assert_eq!(answer, 40);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        // 10 12
        let mut jboxes = parse(INPUT.trim());
        jboxes.construct_circuits(jboxes.min_distances());
        let answer = jboxes.last_conn[0].x * jboxes.last_conn[1].x;
        assert_eq!(answer, 25272);
        Ok(())
    }
}
