use anyhow::{Ok, Result};
use glam::IVec2;
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::ops::Range;

#[derive(Debug, Clone)]
struct RedTileList {
    coords: Vec<IVec2>,
}
impl RedTileList {
    fn new(coords: Vec<IVec2>) -> Self {
        Self { coords }
    }
    fn max_area(&self) -> i64 {
        let mut max_area = 0;
        let len = self.coords.len();
        for i in 0..len {
            for j in i + 1..len {
                let area = ((self.coords[i].x - self.coords[j].x).abs() + 1) as i64
                    * ((self.coords[i].y - self.coords[j].y).abs() + 1) as i64;
                max_area = max_area.max(area);
            }
        }
        max_area
    }
    fn max_area_in_polygon(&self) -> i64 {
        let mut max_area = 0;
        let len = self.coords.len();
        for i in 0..len {
            for j in i + 1..len {
                let area = ((self.coords[i].x - self.coords[j].x).abs() + 1) as i64
                    * ((self.coords[i].y - self.coords[j].y).abs() + 1) as i64;
                if max_area > area {
                    continue;
                }
                if !self.is_smaller_rect_in_polygon(i, j) {
                    continue;
                }
                max_area = area;
            }
        }
        max_area
    }
    fn is_intersects_edge(&self, edge: [IVec2; 2]) -> bool {
        self.coords
            .iter()
            .zip(self.coords.iter().skip(1))
            .map(|(a, b)| [*a, *b])
            .any(|poly_edge| {
                let (ax, ay) = ranges_from_edge(edge);
                let (bx, by) = ranges_from_edge(poly_edge);
                ax.contains(&bx.start) && by.contains(&ay.start)
                    || ay.contains(&by.start) && bx.contains(&ax.start)
            })
    }
    fn is_smaller_rect_in_polygon(&self, i: usize, j: usize) -> bool {
        let sign_x = (self.coords[i].x - self.coords[j].x).signum();
        let sign_y = (self.coords[i].y - self.coords[j].y).signum();
        let edge_ix = [
            IVec2::new(self.coords[i].x - sign_x, self.coords[i].y - sign_y),
            IVec2::new(self.coords[j].x + sign_x, self.coords[i].y - sign_y),
        ];
        let edge_iy = [
            IVec2::new(self.coords[i].x - sign_x, self.coords[i].y - sign_y),
            IVec2::new(self.coords[i].x - sign_x, self.coords[j].y + sign_y),
        ];
        let edge_jx = [
            IVec2::new(self.coords[j].x + sign_x, self.coords[j].y + sign_y),
            IVec2::new(self.coords[i].x - sign_x, self.coords[j].y + sign_y),
        ];
        let edge_jy = [
            IVec2::new(self.coords[j].x + sign_x, self.coords[j].y + sign_y),
            IVec2::new(self.coords[j].x + sign_x, self.coords[i].y - sign_y),
        ];
        [edge_ix, edge_iy, edge_jx, edge_jy]
            .iter()
            .all(|edge| !self.is_intersects_edge(*edge))
    }
    fn find_next_red_tile(&self, cur: IVec2) -> Option<IVec2> {
        self.coords
            .iter()
            .copied()
            .filter(|next| *next != cur)
            .find(|next| cur.x == next.x || cur.y == next.y)
    }
    fn build_polygon(&mut self) {
        let len = self.coords.len();
        let mut polygon = Vec::with_capacity(len);
        let upper_left = self
            .coords
            .iter()
            .copied()
            .min_by(|a, b| match a.x.cmp(&b.x) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => a.y.cmp(&b.y),
            })
            .expect("empty list");
        polygon.push(
            self.coords
                .remove(self.coords.iter().position(|c| *c == upper_left).unwrap()),
        );
        let mut cur = upper_left;
        while let Some(next) = self.find_next_red_tile(cur) {
            let remove_i = self.coords.iter().position(|c| *c == next).unwrap();
            polygon.push(self.coords.remove(remove_i));
            cur = next;
        }
        polygon.push(upper_left);
        self.coords = polygon;
    }
}

fn ranges_from_edge(edge: [IVec2; 2]) -> (Range<i32>, Range<i32>) {
    let [a, b] = edge;
    let px = if a.x < b.x { a.x..b.x } else { b.x..a.x };
    let py = if a.y < b.y { a.y..b.y } else { b.y..a.y };
    (px, py)
}

fn parse(input: &str) -> RedTileList {
    let coords = input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| IVec2::from((x.parse().unwrap(), y.parse().unwrap())))
                .unwrap()
        })
        .collect();
    RedTileList::new(coords)
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/day09-input1.txt")?;
    let mut red_tiles = parse(input.trim());
    let answer = red_tiles.max_area();
    println!("part 1 answer is: {answer}");
    red_tiles.build_polygon();
    let answer = red_tiles.max_area_in_polygon();
    println!("part 2 answer is: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;
    #[test]
    fn part1() -> Result<()> {
        let red_tiles = parse(INPUT.trim());
        let answer = red_tiles.max_area();
        assert_eq!(answer, 50);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let mut red_tiles = parse(INPUT.trim());
        red_tiles.build_polygon();
        let answer = red_tiles.max_area_in_polygon();
        assert_eq!(answer, 24);
        Ok(())
    }
}
