use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;

type Coord = (usize, usize);

struct Region {
    coords: Vec<Coord>,
    pub region_type: char,
}

impl Region {
    pub fn area(&self) -> usize {
        self.coords.len()
    }

    pub fn perimeter(&self) -> usize {
        let nodes = self.coords.iter().cloned().collect::<HashSet<_>>();
        let mut edges = 0;
        for &(y, x) in self.coords.iter() {
            for [dy, dx] in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
                if let (Ok(y), Ok(x)) = (
                    TryInto::<usize>::try_into(y as i32 + dy),
                    TryInto::<usize>::try_into(x as i32 + dx),
                ) {
                    if !nodes.contains(&(y, x)) {
                        edges += 1;
                    }
                } else {
                    edges += 1;
                }
            }
        }
        edges
    }
}

fn traverse(grid: Vec<Vec<char>>) -> Vec<Region> {
    let mut regions = vec![];
    let mut visited = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let region_type = grid[i][j];
            let mut coords = vec![];
            let mut queue = VecDeque::from([(i as i32, j as i32)]);

            while let Some(coord) = queue.pop_front() {
                let (y, x) = coord;
                if y >= 0
                    && x >= 0
                    && y < grid.len() as i32
                    && x < grid[0].len() as i32
                    && grid[y as usize][x as usize] == region_type
                    && visited.insert(coord)
                {
                    coords.push((y as usize, x as usize));

                    for d in [-1, 1] {
                        queue.push_back((y, x + d));
                        queue.push_back((y + d, x));
                    }
                }
            }
            if !coords.is_empty() {
                regions.push(Region {
                    coords,
                    region_type,
                });
            }
        }
    }
    regions
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid = fs::read_to_string("p1.txt")?
        .split_ascii_whitespace()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let regions = traverse(grid);

    println!("{}", regions.len());
    for r in &regions {
        println!(
            "{}: {} x {} = {}",
            r.region_type,
            r.area(),
            r.perimeter(),
            r.area() * r.perimeter()
        );
    }

    let pt1_sum = regions
        .iter()
        .map(|r| r.area() * r.perimeter())
        .sum::<usize>();
    println!("pt1_sum: {pt1_sum}");

    Ok(())
}
