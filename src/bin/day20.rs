use anyhow::Result;
use itertools::Itertools;

fn part01() -> Result<i32> {
    let original_order = std::fs::read_to_string("./data/day20.input")?
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| (line.parse::<i32>().unwrap(), i))
        .collect_vec();
    let mut numbers = original_order.clone();
    // let modulus = numbers.len() as i32 - 1;
    let num_nums = numbers.len() as i32;
    for (n, i) in &original_order {
        let mut current_idx = numbers.iter().position(|(m, j)| m == n && i == j).unwrap() as i32;
        let mut remaining_steps = *n;
        while remaining_steps != 0 {
            let new_idx = if *n > 0 {
                remaining_steps -= 1;
                (current_idx + 1) % num_nums
            } else {
                remaining_steps += 1;
                ((current_idx - 1) + num_nums) % num_nums
            };
            numbers.swap(current_idx as usize, new_idx as usize);
            current_idx = new_idx;
        }
    }
    let idx_of_zero = numbers.iter().position(|n| n.0 == 0).unwrap();
    Ok((1..=3).fold(0, |acc, i| acc + numbers[(idx_of_zero + i * 1000) % num_nums as usize].0))
}

fn part02() -> Result<i64> {
    const DEC_KEY: i64 = 811589153;
    let original_order = std::fs::read_to_string("./data/day20.input")?
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| (line.parse::<i64>().unwrap() * DEC_KEY, i))
        .collect_vec();
    let mut numbers = original_order.clone();
    let modulus = numbers.len() as i64 - 1;
    let num_nums = numbers.len() as i32;
    for _ in 0..10 {
        for (n, i) in &original_order {
            let mut current_idx = numbers.iter().position(|(m, j)| m == n && i == j).unwrap() as i32;
            let mut remaining_steps = *n % modulus;
            while remaining_steps != 0 {
                let new_idx = if *n > 0 {
                    remaining_steps -= 1;
                    (current_idx + 1) % num_nums
                } else {
                    remaining_steps += 1;
                    ((current_idx - 1) + num_nums) % num_nums
                };
                numbers.swap(current_idx as usize, new_idx as usize);
                current_idx = new_idx;
            }
        }
    }
    let idx_of_zero = numbers.iter().position(|n| n.0 == 0).unwrap();
    Ok((1..=3).fold(0, |acc, i| acc + numbers[(idx_of_zero + i * 1000) % num_nums as usize].0))
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
