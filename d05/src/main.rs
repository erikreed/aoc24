use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn is_valid(page_orders: &HashMap<i32, Vec<i32>>, nums: &Vec<i32>) -> bool {
    let mut constraints = HashSet::new();
    let mut valid = true;
    for n in nums.iter().rev() {
        if constraints.contains(n) {
            valid = false;
            break;
        }
        page_orders
            .get(n)
            .map(|o| constraints.extend(o.iter().cloned()));
    }
    valid
}

fn make_valid(page_orders: &HashMap<i32, Vec<i32>>, nums: &Vec<i32>) -> Vec<i32> {
    let mut constraints = HashMap::new();
    let mut new_nums = nums.clone();
    loop {
        constraints.clear();
        let mut valid = true;

        for (i, n) in new_nums.iter().enumerate().rev() {
            if let Some(&k) = constraints.get(n) {
                valid = false;
                new_nums.swap(i, k);
                break;
            }
            page_orders
                .get(n)
                .map(|o| constraints.extend(o.iter().map(|&c| (c, i))));
        }
        if valid {
            break new_nums;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("p1.txt")?;

    let (p1, p2) = text.split_once("\n\n").unwrap();
    let mut page_orders = HashMap::<i32, Vec<i32>>::new();
    for line in p1.lines() {
        let (order_a, order_b) = line.split_once('|').unwrap();
        let (order_a, order_b) = (order_a.parse::<i32>()?, order_b.parse::<i32>()?);
        page_orders.entry(order_a).or_default().push(order_b);
    }

    let mut p1_sum = 0;
    let mut p2_sum = 0;

    for line in p2.lines() {
        let nums = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_valid(&page_orders, &nums) {
            p1_sum += nums[nums.len() / 2];
        } else {
            let new_nums = make_valid(&page_orders, &nums);
            p2_sum += new_nums[new_nums.len() / 2];
        }
    }

    println!("p1_sum: {p1_sum}");
    println!("p2_sum: {p2_sum}");

    Ok(())
}
