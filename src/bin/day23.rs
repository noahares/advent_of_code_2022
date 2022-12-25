use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn nw(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0 - 1, pos.1 - 1)
}
fn n(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0 - 1, pos.1)
}
fn ne(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0 - 1, pos.1 + 1)
}
fn w(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0, pos.1 - 1)
}
fn e(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0, pos.1 + 1)
}
fn sw(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0 + 1, pos.1 - 1)
}
fn s(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0 + 1, pos.1)
}
fn se(pos: &(i32, i32)) -> (i32, i32) {
    (pos.0 + 1, pos.1 + 1)
}

#[allow(dead_code)]
fn print_board(board: &HashMap<(i32, i32), u32>) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    for elve in board.iter() {
        min_x = min_x.min(elve.0.0);
        max_x = max_x.max(elve.0.0);
        min_y = min_y.min(elve.0.1);
        max_y = max_y.max(elve.0.1);
    }
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            if board.contains_key(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_covered_ground(board: &HashMap<(i32, i32), u32>) -> u32 {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    for elve in board.iter() {
        min_x = min_x.min(elve.0.0);
        max_x = max_x.max(elve.0.0);
        min_y = min_y.min(elve.0.1);
        max_y = max_y.max(elve.0.1);
    }
    (max_x.abs_diff(min_x) + 1) * (max_y.abs_diff(min_y) + 1) - board.len() as u32
}

fn main() -> Result<()> {
    let mut elve_id = 0_u32;
    let mut board: HashMap<(i32, i32), u32> = std::fs::read_to_string("./data/day23.input")?
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter_map(|(j, c)| match c {
                    '#' => {
                        let result = ((i as i32, j as i32), elve_id);
                        elve_id += 1;
                        Some(result)
                    }
                    _ => None,
                })
                .collect_vec()
        })
        .collect();
        let mut consideration_order = vec!['N', 'S', 'W', 'E'];
        for i in 0.. {
            let mut propositions: HashMap<(i32, i32), u32> = HashMap::new();
            let mut old_pos = vec![(0_i32, 0_i32); board.len()];
            for elve in board.iter() {
                let (pos, id) = elve;
                old_pos[*id as usize] = *pos;
                let neighbors = vec![nw(pos), n(pos), ne(pos), w(pos), e(pos), sw(pos), s(pos), se(pos)];
                if !neighbors.iter().any(|n| board.contains_key(n)) {
                    propositions.insert(*pos, *id);
                    continue;
                }
                let mut considered_direction = consideration_order.iter().clone();
                loop {
                    let direction = considered_direction.next();
                    match direction {
                        Some('N') => {
                            if !vec![nw(pos), n(pos), ne(pos)].iter().any(|n| board.contains_key(n)) {
                                if let std::collections::hash_map::Entry::Vacant(e) = propositions.entry(n(pos)) {
                                    e.insert(*id);
                                } else {
                                    let old_elve = propositions.remove_entry(&n(pos)).unwrap();
                                    propositions.insert(old_pos[old_elve.1 as usize], old_elve.1);
                                    propositions.insert(*pos, *id);
                                }
                                break;
                            }
                        },
                        Some('S') => {
                            if !vec![sw(pos), s(pos), se(pos)].iter().any(|n| board.contains_key(n)) {
                                if let std::collections::hash_map::Entry::Vacant(e) = propositions.entry(s(pos)) {
                                    e.insert(*id);
                                } else {
                                    let old_elve = propositions.remove_entry(&s(pos)).unwrap();
                                    propositions.insert(old_pos[old_elve.1 as usize], old_elve.1);
                                    propositions.insert(*pos, *id);
                                }
                                break;
                            }
                        },
                        Some('W') => {
                            if !vec![nw(pos), w(pos), sw(pos)].iter().any(|n| board.contains_key(n)) {
                                if let std::collections::hash_map::Entry::Vacant(e) = propositions.entry(w(pos)) {
                                    e.insert(*id);
                                } else {
                                    let old_elve = propositions.remove_entry(&w(pos)).unwrap();
                                    propositions.insert(old_pos[old_elve.1 as usize], old_elve.1);
                                    propositions.insert(*pos, *id);
                                }
                                break;
                            }
                        },
                        Some('E') => {
                            if !vec![ne(pos), e(pos), se(pos)].iter().any(|n| board.contains_key(n)) {
                                if let std::collections::hash_map::Entry::Vacant(e) = propositions.entry(e(pos)) {
                                    e.insert(*id);
                                } else {
                                    let old_elve = propositions.remove_entry(&e(pos)).unwrap();
                                    propositions.insert(old_pos[old_elve.1 as usize], old_elve.1);
                                    propositions.insert(*pos, *id);
                                }
                                break;
                            }
                        },
                        None => {
                            propositions.insert(*pos, *id);
                            break;
                        },
                        Some(_) => unreachable!()
                    }
                }
            }
            consideration_order.rotate_left(1);
            if i == 10 {
                println!("part1: {}", get_covered_ground(&board));
            }
            if board == propositions {
                println!("part2: {}", i + 1);
                break;
            }
            std::mem::swap(&mut board, &mut propositions);
        }
    Ok(())
}
