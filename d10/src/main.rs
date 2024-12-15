use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn dfs(grid: &Vec<Vec<u8>>, origin_y: usize, origin_x: usize, allow_repeats: bool) -> u32 {
    let mut visited = HashSet::new();
    let mut stack = vec![(origin_y, origin_x)];
    let mut count = 0;

    while let Some(next) = stack.pop() {
        if !allow_repeats && !visited.insert(next) {
            continue;
        }
        let (next_y, next_x) = next;
        let c = grid[next_y][next_x];

        if c == 9 {
            count += 1;
            continue;
        }

        let neighbors = [-1, 1]
            .iter()
            .flat_map(|&o| {
                [
                    (o + next_y as i32, next_x as i32),
                    (next_y as i32, next_x as i32 + o),
                ]
            })
            .filter(|&(y, x)| y >= 0 && x >= 0)
            .map(|(y, x)| (y as usize, x as usize))
            .filter_map(|(y, x)| grid.get(y).and_then(|row| row.get(x).map(|&c| (y, x, c))));

        for (y, x, n) in neighbors {
            if n.checked_sub(c).unwrap_or_default() == 1 {
                stack.push((y, x));
            }
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid = fs::read_to_string("p1.txt")?;
    let grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(255) as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut p1_sum = 0u32;
    let mut p2_sum = 0u32;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                p1_sum += dfs(&grid, i, j, false);
                p2_sum += dfs(&grid, i, j, true);
            }
        }
    }

    println!("p1_sum: {p1_sum}");
    println!("p2_sum: {p2_sum}");

    Ok(())
}
