use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_bounds(grid: &[Vec<char>], x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("p1.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut type_locs: HashMap<char, Vec<_>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                type_locs.entry(c).or_default().push((i as i32, j as i32));
            }
        }
    }

    let mut occupied_p1 = HashSet::new();
    let mut occupied_p2 = HashSet::new();
    for (_c, positions) in type_locs.iter() {
        for pair in positions.iter().permutations(2) {
            let ((y1, x1), (y2, x2)) = (*pair[0], *pair[1]);
            let dy = y2 - y1;
            let dx = x2 - x1;

            for (sign, mut x, mut y) in [(-1, x1, y1), (1, x2, y2)] {
                occupied_p1.insert((x + dx * sign, y + dy * sign));
                loop {
                    if check_bounds(&grid, x, y) {
                        occupied_p2.insert((x, y));
                    } else {
                        break;
                    }
                    x += dx * sign;
                    y += dy * sign;
                }
            }
        }
    }
    occupied_p1.retain(|&(x, y)| check_bounds(&grid, x, y));

    println!("p1: {}", occupied_p1.len());
    println!("p2: {}", occupied_p2.len());

    Ok(())
}
