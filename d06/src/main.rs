use std::collections::HashSet;
use crate::Direction::*;
use std::error::Error;
use std::fs;

type Grid = Vec<Vec<bool>>;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    pub fn intersects(&self, obstacles: &Grid) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }
        obstacles
            .get(self.y as usize)
            .and_then(|row| row.get(self.x as usize))
            .cloned()
            .unwrap_or_default()
    }

    pub fn rotate_right(&mut self) {
        self.direction = self.direction.rotate_right();
    }

    pub fn advance(&self) -> Self {
        match &self.direction {
            &direction @ Up => Self {
                x: self.x,
                y: self.y - 1,
                direction,
            },
            &direction @ Down => Self {
                x: self.x,
                y: self.y + 1,
                direction,
            },
            &direction @ Left => Self {
                x: self.x - 1,
                y: self.y,
                direction,
            },
            &direction @ Right => Self {
                x: self.x + 1,
                y: self.y,
                direction,
            },
        }
    }
}

fn traverse(obstacles: &Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>, start_position: Position) -> bool {
    let mut cycle_check = HashSet::new();

    let mut current_position = start_position;
    loop {
        if !cycle_check.insert(current_position) {
            return false;
        }
        if current_position.x < 0 || current_position.y < 0 {
            break true;
        }
        let pos = visited
            .get_mut(current_position.y as usize)
            .and_then(|row| row.get_mut(current_position.x as usize));

        if let Some(pos) = pos {
            *pos = true;
        } else {
            break true;
        }

        let mut i = 0;
        current_position = loop {
            let advanced = current_position.advance();
            if !advanced.intersects(&obstacles) {
                break advanced;
            }
            current_position.rotate_right();
            i += 1;
            assert!(i < 4, "inf loop detected");
        };
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("p1.txt")?;

    let mut obstacles = text
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; obstacles.len()]; obstacles[0].len()];

    let start_pos = text.lines().enumerate()
        .map(|(i, line)| (i, line.find('^')))
        .filter_map(|(i, p)| p.map(|p| (i, p)))
        .next()
        .unwrap();

    let start_pos = Position {
        direction: Up,
        y: start_pos.0 as i32,
        x: start_pos.1 as i32,
    };

    traverse(&obstacles, &mut visited, start_pos);

    let count: usize = visited.iter().flat_map(|r| r.iter()).map(|&e|e as usize).sum();
    println!("part 1: {count}");

    let mut part2_count = 0;
    for i in 0..obstacles.len() {
        for j in 0..obstacles[i].len() {
            if !obstacles[i][j] && (start_pos.y != i as i32 || start_pos.x != j as i32) {
                obstacles[i][j] = true;
                if !traverse(&obstacles, &mut visited, start_pos) {
                    part2_count += 1;
                }
                obstacles[i][j] = false;
            }
        }
    }
    println!("part 2: {part2_count}");

    Ok(())
}
