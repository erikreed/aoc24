use regex::Regex;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("p1.txt")?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let mut p1_sum = 0;
    let mut p2_sum = 0;

    let start_idx = text
        .match_indices("do()")
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    let end_idx = text
        .match_indices("don't()")
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();

    let is_enabled_at_idx = |idx: usize| {
        if idx < end_idx[0] {
            return true;
        }
        let last_enabled = *start_idx.get(start_idx.binary_search(&idx).err().unwrap() - 1).unwrap_or(&0);
        let last_disabled = *end_idx.get(end_idx.binary_search(&idx).err().unwrap() - 1).unwrap_or(&0);
        last_enabled > last_disabled
    };

    for (i, (_, [a, b])) in re
        .captures_iter(&text)
        .map(|c| (c.get(0).unwrap().start(), c.extract()))
    {
        let a = a.parse::<i32>()?;
        let b = b.parse::<i32>()?;
        p1_sum += a * b;

        if is_enabled_at_idx(i) {
            p2_sum += a * b;
        }
    }

    println!("p1_sum: {p1_sum}");
    println!("p2_sum: {p2_sum}");

    Ok(())
}
