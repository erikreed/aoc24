use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("in1.txt")?;
    let reader = BufReader::new(file);

    let (mut v1, mut v2) = (Vec::new(), Vec::new());
    for line in reader.lines().map(|l| l.unwrap()) {
        let (a, b) = line.split_once(' ').unwrap();
        v1.push(a.trim().parse::<i32>().unwrap());
        v2.push(b.trim().parse::<i32>().unwrap());
    }
    v1.sort();
    v2.sort();

    let mut p1_sum = 0;
    for (a, b) in v1.iter().zip(v2.iter()) {
        p1_sum += (a - b).abs();
    }
    println!("p1 sum: {}", p1_sum);

    let mut counts = HashMap::<i32, i32>::new();
    for b in v2 {
        counts.entry(b).and_modify(|c| { *c = *c + 1 }).or_insert(1);
    }

    let mut p2_sum = 0;
    for a in v1 {
        p2_sum += a * counts.entry(a).or_default().deref();
    }
    println!("p2_sum: {}", p2_sum);

    Ok(())
}