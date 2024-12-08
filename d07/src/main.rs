use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    pub fn eval(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => a * 10_i64.pow(b.ilog10() + 1) + b,
        }
    }
}

fn check_possible(total: i64, nums: &[i64], available_ops: &[Operator]) -> bool {
    let permutation_count = available_ops.len().pow((nums.len() - 1) as u32);
    let mut ops = vec![Operator::Add; nums.len() - 1];
    for mut n in 0..permutation_count {
        for op in &mut ops {
            *op = available_ops[n % available_ops.len()];
            n /= available_ops.len();
        }
        let computed = nums
            .iter()
            .skip(1)
            .zip(ops.iter())
            .fold(nums[0], |a, (&b, op)| op.eval(a, b));

        if computed == total {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("p1.txt")?;
    let reader = BufReader::new(file);

    let mut p1_sum = 0i64;
    let mut p2_sum = 0i64;
    let p1_ops = vec![Operator::Add, Operator::Multiply];
    let p2_ops = vec![Operator::Add, Operator::Multiply, Operator::Concat];

    for line in reader.lines().map(|l| l.unwrap()) {
        let (a, b) = line.split_once(':').unwrap();
        let total = a.parse::<i64>().unwrap();
        let nums = b
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        if check_possible(total, &nums, &p1_ops) {
            p1_sum += total;
        }
        if check_possible(total, &nums, &p2_ops) {
            p2_sum += total;
        }
    }

    println!("p1_sum: {p1_sum}");
    println!("p2_sum: {p2_sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(Operator::Concat.eval(120, 256), 120256);
        assert_eq!(Operator::Concat.eval(10, 1), 101);
        assert_eq!(Operator::Concat.eval(1, 50), 150);
    }
}
