use anyhow::Result;
use std::collections::HashSet;

fn get_range_iter_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    if b > a {
        let vec: Vec<usize> = (a..=b).collect();
        vec.into_iter()
    } else {
        let vec: Vec<usize> = (b..=a).rev().collect();
        vec.into_iter()
    }
}

fn part01() -> Result<u32> {
    let input: Vec<Vec<(usize, usize)>> = std::fs::read_to_string("./data/day14.input")?
        .trim()
        .lines()
        .map(|line| line.split(" -> ")
             .map(|pos| pos.split_once(',').unwrap())
             .map(|(a,b)| (a.parse().unwrap(), b.parse().unwrap()))
             .collect())
        .collect();
    let mut blocked_positions: HashSet<(usize, usize)> = HashSet::new();
    let sand_origin = (500_usize, 0_usize);

    let mut down_bound: usize = 0;
    let mut side_bounds: (usize, usize) = (500, 500);
    for rock_piece in input.iter() {
        for b in 0..rock_piece.len()-1 {
            for i in get_range_iter_inclusive(rock_piece[b].0, rock_piece[b+1].0) {
                side_bounds = (side_bounds.0.min(i), side_bounds.1.max(i));
                blocked_positions.insert((i, rock_piece[b].1));
            }
            for j in get_range_iter_inclusive(rock_piece[b].1, rock_piece[b+1].1) {
                down_bound = down_bound.max(j);
                blocked_positions.insert((rock_piece[b].0, j));
            }
        }
    }
    let mut num_sand_units = 0;
    loop {
        let mut sand_pos = sand_origin;
        while (side_bounds.0..=side_bounds.1).contains(&sand_pos.0) && sand_pos.1 < down_bound {
            sand_pos.1 += 1;
            if !blocked_positions.contains(&sand_pos) { continue; }
            sand_pos.0 -= 1;
            if !blocked_positions.contains(&sand_pos) { continue; }
            sand_pos.0 += 2;
            if !blocked_positions.contains(&sand_pos) { continue; }
            sand_pos = (sand_pos.0-1, sand_pos.1-1);
            break;
        }
        if (side_bounds.0..=side_bounds.1).contains(&sand_pos.0) && sand_pos.1 < down_bound {
            blocked_positions.insert(sand_pos);
            num_sand_units += 1;
        } else { break; }
    }
    Ok(num_sand_units)
}

fn part02() -> Result<u32> {
    let input: Vec<Vec<(usize, usize)>> = std::fs::read_to_string("./data/day14.input")?
        .trim()
        .lines()
        .map(|line| line.split(" -> ")
             .map(|pos| pos.split_once(',').unwrap())
             .map(|(a,b)| (a.parse().unwrap(), b.parse().unwrap()))
             .collect())
        .collect();
    let mut blocked_positions: HashSet<(usize, usize)> = HashSet::new();
    let sand_origin = (500_usize, 0_usize);

    let mut down_bound: usize = 0;
    for rock_piece in input.iter() {
        for b in 0..rock_piece.len()-1 {
            for i in get_range_iter_inclusive(rock_piece[b].0, rock_piece[b+1].0) {
                blocked_positions.insert((i, rock_piece[b].1));
            }
            for j in get_range_iter_inclusive(rock_piece[b].1, rock_piece[b+1].1) {
                down_bound = down_bound.max(j);
                blocked_positions.insert((rock_piece[b].0, j));
            }
        }
    }
    down_bound += 1;
    let mut num_sand_units = 0;
    loop {
        let mut sand_pos = sand_origin;
        while sand_pos.1 < down_bound {
            sand_pos.1 += 1;
            if !blocked_positions.contains(&sand_pos) { continue; }
            sand_pos.0 -= 1;
            if !blocked_positions.contains(&sand_pos) { continue; }
            sand_pos.0 += 2;
            if !blocked_positions.contains(&sand_pos) { continue; }
            sand_pos = (sand_pos.0-1, sand_pos.1-1);
            break;
        }
        blocked_positions.insert(sand_pos);
        num_sand_units += 1;
        if sand_pos == sand_origin { break; }
    }
    Ok(num_sand_units)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
