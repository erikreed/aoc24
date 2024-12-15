use std::collections::HashMap;
use std::error::Error;
use std::fs;

struct Sequence {
    pub current: Vec<u64>,
    next: Vec<u64>,
}

fn step(d: u64) -> Vec<u64> {
    let n_digits = (d as f32).log10().floor() as u32 + 1;
    if d == 0 {
        vec![1]
    } else if n_digits % 2 == 0 {
        let n = n_digits / 2;
        vec![d / 10_u64.pow(n), d % 10_u64.pow(n)]
    } else {
        vec![d * 2024]
    }
}

impl Sequence {
    fn new(init: Vec<u64>) -> Self {
        Self {
            next: Vec::with_capacity(init.capacity()),
            current: init,
        }
    }

    fn step(&mut self) {
        self.next.clear();
        for &d in &self.current {
            let n_digits = (d as f32).log10().floor() as u32 + 1;
            if d == 0 {
                self.next.push(1);
            } else if n_digits % 2 == 0 {
                let n = n_digits / 2;
                self.next.push(d / 10_u64.pow(n));
                self.next.push(d % 10_u64.pow(n));
            } else {
                self.next.push(d * 2024);
            }
        }

        std::mem::swap(&mut self.current, &mut self.next);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = fs::read_to_string("p1.txt")?
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1_sum: usize = input
        .iter()
        .map(|&n| Sequence::new(vec![n]))
        .map(|mut s| {
            for _ in 0..25 {
                s.step();
            }
            s
        })
        .map(|s| s.current.len())
        .sum();

    println!("part 1: {}", part1_sum);

    let mut counts = HashMap::new();
    for n in input {
        *counts.entry(n).or_insert(0) += 1;
    }
    for _ in 0..75 {
        let mut new_counts = HashMap::new();

        for (&k, &v) in counts.iter() {
            let new_k = step(k);
            for little_k in new_k {
                *new_counts.entry(little_k).or_insert(0) += v;
            }
        }
        counts = new_counts;
    }

    let pt2 = counts.values().sum::<u64>();
    println!("pt2: {}", pt2);

    Ok(())
}
