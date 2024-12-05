use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut all_asc = true;
    let mut all_desc = true;

    for (&a, &b) in nums.iter().zip(nums.iter().skip(1)) {
        let d = (b - a).abs();
        if d > 3 || d < 1 {
            safe = false;
            break;
        }
        if b > a {
            all_desc = false;
        }
        if a > b {
            all_asc = false;
        }
    }
    safe = safe && (all_asc || all_desc);
    safe
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("p1.txt")?;
    let reader = BufReader::new(file);

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        let nums = line
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_safe(&nums) {
            p1_sum += 1;
            p2_sum += 1;
        } else {
            for i in 0..nums.len() {
                let mut nums = nums.clone();
                nums.remove(i);
                if is_safe(&nums) {
                    p2_sum += 1;
                    break;
                }
            }
        }
    }
    println!("p1_sum: {p1_sum}");
    println!("p2_sum: {p2_sum}");

    Ok(())
}
