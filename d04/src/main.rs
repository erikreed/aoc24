use std::error::Error;
use std::fs;
use std::io::BufRead;

const TARGET_STRING: &[u8] = "XMAS".as_bytes();
type Grid = Vec<Vec<u8>>;

fn grid_get(grid: &Grid, x: i32, y: i32) -> Option<u8> {
    if x < 0 || y < 0 {
        None
    } else {
        grid.get(x as usize)
            .and_then(|r| r.get(y as usize).cloned())
    }
}

fn _try_position_any_order(grid: &Grid, x: usize, y: usize, mut current: String) -> bool {
    if current.as_bytes() == TARGET_STRING {
        return true;
    }

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let x: Result<usize, _> = (x as i32 + i).try_into();
            let y: Result<usize, _> = (y as i32 + j).try_into();
            if let (Ok(x), Ok(y)) = (x, y) {
                if let Some(&c) = grid.get(x).and_then(|r| r.get(y)) {
                    if TARGET_STRING[current.len()] == c {
                        current.push(c as char);
                        if _try_position_any_order(grid, x, y, current.clone()) {
                            return true;
                        }
                        current.pop();
                    }
                }
            }
        }
    }

    false
}

fn count_at_position(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;

    'outer: for sign in [-1, 1] {
        // check up/down
        for i in 0..TARGET_STRING.len() {
            let x = x as i32 + i as i32 * sign;
            let y = y as i32;

            let c = grid_get(grid, x, y);
            if c.map(|c| c != TARGET_STRING[i]).unwrap_or(true) {
                continue 'outer;
            }
        }
        count += 1;
    }

    'outer: for sign in [-1, 1] {
        // check left/right
        for i in 0..TARGET_STRING.len() {
            let x = x as i32;
            let y = y as i32 + i as i32 * sign;

            let c = grid_get(grid, x, y);
            if c.map(|c| c != TARGET_STRING[i]).unwrap_or(true) {
                continue 'outer;
            }
        }
        count += 1;
    }

    'outer: for sign in [-1, 1] {
        // check diagonal
        for i in 0..TARGET_STRING.len() {
            let x = x as i32 + i as i32 * sign;
            let y = y as i32 + i as i32 * sign;

            let c = grid_get(grid, x, y);
            if c.map(|c| c != TARGET_STRING[i]).unwrap_or(true) {
                continue 'outer;
            }
        }
        count += 1;
    }

    'outer: for sign in [-1, 1] {
        // check diagonal
        for i in 0..TARGET_STRING.len() {
            let x = x as i32 + i as i32 * sign;
            let y = y as i32 + i as i32 * -sign;

            let c = grid_get(grid, x, y);
            if c.map(|c| c != TARGET_STRING[i]).unwrap_or(true) {
                continue 'outer;
            }
        }
        count += 1;
    }

    count
}

fn contains_x_mas(grid: &Grid, x: usize, y: usize) -> bool {
    let x = x as i32;
    let y = y as i32;

    if grid_get(grid, x, y).map(|c| c != b'A').unwrap_or(true) {
        return false;
    }

    let ul = grid_get(grid, x - 1, y - 1);
    let ur = grid_get(grid, x - 1, y + 1);
    let bl = grid_get(grid, x + 1, y - 1);
    let br = grid_get(grid, x + 1, y + 1);
    let s = b'M' + b'S';
    ul.unwrap_or_default() + br.unwrap_or_default() == s
        && ur.unwrap_or_default() + bl.unwrap_or_default() == s
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("p1.txt")?;
    let grid: Grid = text.lines().map(|line| line.as_bytes().to_vec()).collect();

    let mut p1_count = 0;
    let mut p2_count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            p1_count += count_at_position(&grid, i, j);
            p2_count += contains_x_mas(&grid, i, j) as usize;
        }
    }

    println!("p1_count: {p1_count}");
    println!("p2_count: {p2_count}");

    Ok(())
}
