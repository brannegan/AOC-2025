use anyhow::{Ok, Result};
use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, algo};

use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::RandomState;

#[derive(Debug, Clone)]
struct AttachedDevices {
    data: HashMap<[char; 3], Vec<[char; 3]>>,
    graph: Graph<[char; 3], ()>,
}

impl AttachedDevices {
    fn new(hm: HashMap<[char; 3], Vec<[char; 3]>>) -> Self {
        Self {
            data: hm,
            graph: Graph::new(),
        }
    }
    fn build_graph(mut self) -> Self {
        self.data
            .iter()
            .flat_map(|(node, connected)| connected.iter().map(move |other| (node, other)))
            .for_each(|(a, b)| {
                let a = self
                    .graph
                    .node_indices()
                    .find(|ni| self.graph[*ni] == *a)
                    .unwrap_or_else(|| self.graph.add_node(*a));
                let b = self
                    .graph
                    .node_indices()
                    .find(|ni| self.graph[*ni] == *b)
                    .unwrap_or_else(|| self.graph.add_node(*b));
                self.graph.add_edge(a, b, ());
            });
        self
    }
    fn path_count_you_out(&self) -> usize {
        let you = self
            .graph
            .node_indices()
            .find(|ni| self.graph[*ni] == ['y', 'o', 'u'])
            .expect("you not found");
        let out = self
            .graph
            .node_indices()
            .find(|ni| self.graph[*ni] == ['o', 'u', 't'])
            .expect("out not found");
        algo::all_simple_paths::<Vec<_>, _, RandomState>(&self.graph, you, out, 1, None).count()
    }
    fn path_count(&self, topo: &[NodeIndex]) -> usize {
        let mut path_counter_map: HashMap<[char; 3], usize> =
            HashMap::from_iter(topo.iter().map(|ni| (self.graph[*ni], 0)));
        path_counter_map
            .entry(self.graph[topo[0]])
            .and_modify(|counter| *counter += 1);
        for source_ni in topo {
            for target_ni in self.graph.neighbors(*source_ni) {
                let source_path_count = path_counter_map[&self.graph[*source_ni]];
                path_counter_map
                    .entry(self.graph[target_ni])
                    .and_modify(|counter| *counter += source_path_count);
            }
        }
        path_counter_map[&self.graph[*topo.last().unwrap()]]
    }
    fn path_count_srv_out(&self) -> usize {
        let toposorted = toposort(&self.graph, None).expect("cycle in graph");
        let svr = toposorted
            .iter()
            .position(|ni| self.graph[*ni] == ['s', 'v', 'r'])
            .expect("svr not found");
        let fft = toposorted
            .iter()
            .position(|ni| self.graph[*ni] == ['f', 'f', 't'])
            .expect("fft not found");
        let dac = toposorted
            .iter()
            .position(|ni| self.graph[*ni] == ['d', 'a', 'c'])
            .expect("dac not found");
        let out = toposorted
            .iter()
            .position(|ni| self.graph[*ni] == ['o', 'u', 't'])
            .expect("out not found");
        self.path_count(&toposorted[svr..=fft])
            * self.path_count(&toposorted[fft..=dac])
            * self.path_count(&toposorted[dac..=out])
    }
}

fn parse(input: &str) -> AttachedDevices {
    let parsed = input
        .lines()
        .filter_map(|line| {
            line.split_once(":").map(|(source, targets)| {
                let s: [char; 3] = source.chars().collect_array::<3>().unwrap();
                let t: Vec<[char; 3]> = targets
                    .split_whitespace()
                    .map(|target| target.chars().collect_array::<3>().unwrap())
                    .collect();
                (s, t)
            })
        })
        .collect();
    AttachedDevices::new(parsed)
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day11-input1.txt")?;
    let devices = parse(input.trim());
    let devices = devices.build_graph();
    let answer = devices.path_count_you_out();
    println!("part 1 answer is: {answer}");
    let answer = devices.path_count_srv_out();
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;
    const INPUT2: &str = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn part1() -> Result<()> {
        let devices = parse(INPUT.trim());
        let answer = devices.build_graph().path_count_you_out();
        assert_eq!(answer, 5);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let devices = parse(INPUT2.trim());
        let answer = devices.build_graph().path_count_srv_out();
        assert_eq!(answer, 2);
        Ok(())
    }
}
