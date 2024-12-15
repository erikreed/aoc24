use std::error::Error;
use std::fs;

const FREE_SPACE: usize = usize::MAX;

fn compute_checksum(fs: &[usize]) -> usize {
    let checksum: usize = fs
        .iter()
        .enumerate()
        .filter(|&(_i, b)| b != &FREE_SPACE)
        .map(|(i, &b)| i * b)
        .sum();
    checksum
}

fn p1_defrag(fs: &mut [usize]) {
    let mut start_idx = 0;
    let mut end_idx = fs.len() - 1;
    while start_idx < end_idx {
        let (a, b) = (fs[start_idx], fs[end_idx]);
        if a != FREE_SPACE {
            start_idx += 1;
        } else if b == FREE_SPACE {
            end_idx -= 1;
        } else {
            fs.swap(start_idx, end_idx);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("p1.txt")?;

    let mut fs: Vec<usize> = vec![];
    // pos, len, value
    let mut used = vec![];
    // pos, len
    let mut free = vec![];

    for (i, c) in text.chars().enumerate() {
        let c = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            let blk_id = i / 2;
            used.push((fs.len(), c, blk_id));
            for _ in 0..c {
                fs.push(blk_id);
            }
        } else {
            free.push((fs.len(), c));
            fs.extend((0..c).map(|_| FREE_SPACE));
        }
    }

    p1_defrag(&mut fs);
    let checksum = compute_checksum(&fs);
    println!("part1 sum: {checksum}");

    let mut fs_p2 = vec![FREE_SPACE; fs.len()];
    'block: for b in used.into_iter().rev() {
        for f in free.iter_mut() {
            if b.0 < f.0 {
                break;
            }
            if b.1 <= f.1 {
                fs_p2[f.0..f.0 + b.1].fill(b.2);
                f.0 += b.1;
                f.1 -= b.1;
                continue 'block;
            }
        }
        fs_p2[b.0..b.0 + b.1].fill(b.2);
    }

    let checksum = compute_checksum(&fs_p2);
    // 8532024062389 too high
    println!("part2 sum: {checksum}");

    Ok(())
}
