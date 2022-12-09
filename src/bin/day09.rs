use anyhow::Result;
use std::collections::HashSet;

fn part01() -> Result<u32> {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut tail: (i32, i32) = (0, 0);
    let mut head: (i32, i32) = (0, 0);
    visited_positions.insert(tail);
    std::fs::read_to_string("./data/day09.input")?
        .trim()
        .split("\n")
        .map(|line| line.split_once(" ").map(|(direction, step)| (direction, step.parse::<u32>().unwrap())).unwrap())
        .for_each(|(direction, step)| {
            for _ in 0..step {
                let old_head = head;
                match direction {
                    "U" => head.0 -= 1,
                    "D" => head.0 += 1,
                    "L" => head.1 -= 1,
                    "R" => head.1 += 1,
                    _ => ()
                }
                if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                    tail = old_head;
                    visited_positions.insert(tail);
                }
            }
        });
    Ok(visited_positions.len() as u32)
}

fn part02() -> Result<u32> {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    visited_positions.insert(rope[9]);
    std::fs::read_to_string("./data/day09.input")?
        .trim()
        .split("\n")
        .map(|line| line.split_once(" ").map(|(direction, step)| (direction, step.parse::<u32>().unwrap())).unwrap())
        .for_each(|(direction, step)| {
            for _ in 0..step {
                match direction {
                    "U" => rope[0].0 += 1,
                    "D" => rope[0].0 -= 1,
                    "R" => rope[0].1 += 1,
                    "L" => rope[0].1 -= 1,
                    _ => ()
                }
                for i in 1..rope.len() {
                    let  (a, b) = (rope[i-1].0.abs_diff(rope[i].0), rope[i-1].1.abs_diff(rope[i].1));
                    match (a, b) {
                        _ if a <= 1 && b <= 1 => (),
                        _ => {
                            rope[i].0 += (rope[i-1].0 - rope[i].0).clamp(-1, 1) as i32;
                            rope[i].1 += (rope[i-1].1 - rope[i].1).clamp(-1, 1) as i32;
                        },
                    }
                }
                visited_positions.insert(rope[9]);
            }
        });
    Ok(visited_positions.len() as u32)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
