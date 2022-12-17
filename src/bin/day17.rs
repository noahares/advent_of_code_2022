use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Rock {
    shape: Vec<(i32, i32)>,
    bottom: i32,
    left: i32,
    right: i32,
    height: usize
}

impl Rock {
    fn translate(self: &mut Self, (x, y): (i32, i32)) -> () {
        for pos in &mut self.shape {
            *pos = (pos.0 + x, pos.1 + y);
        }
        self.bottom += y;
        self.left += x;
        self.right += x;
    }
}

fn rock_generator(n: u32) -> impl std::iter::Iterator<Item = Rock> {
    let mut num_rocks = 0;
    std::iter::from_fn(move || {
        if num_rocks >= n { return None }
        let shape = num_rocks % 5;
        num_rocks += 1;
        match shape {
            0 => Some(Rock {shape: vec![(0,0), (1,0), (2,0), (3,0)], bottom: 0, left: 0, right: 3, height: 1}),
            1 => Some(Rock {shape: vec![(0,1), (1,1), (2,1), (1,0), (1, 2)], bottom: 0, left: 0, right: 2, height: 3}),
            2 => Some(Rock {shape: vec![(0,0), (1,0), (2,0), (2,1), (2, 2)], bottom: 0, left: 0, right: 2, height: 3}),
            3 => Some(Rock {shape: vec![(0,0), (0,1), (0,2), (0,3)], bottom: 0, left: 0, right: 0, height: 4}),
            4 => Some(Rock {shape: vec![(0,0), (0,1), (1,0), (1,1)], bottom: 0, left: 0, right: 1, height: 2}),
            _ => unreachable!()
        }
    })
}

fn get_direction(wind: &mut impl Iterator<Item = i32>, rock: &Rock) -> (i32, i32) {
    match wind.next() {
        Some(-1) if rock.left > 0 => (-1, -1),
        Some(1) if rock.right < 6 => (1, -1),
        Some(_) => (0, -1),
        None => unreachable!()
    }

}

fn part01() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day17.input")?
        .trim()
        .chars()
        .map(|char| match char {
            '<' => -1,
            '>' => 1,
            _ => unreachable!()
        })
        .collect::<Vec<i32>>();
        let mut wind = input
        .into_iter()
        .cycle();
    let total_num_rocks = 2022;
    let cave_width = 7;
    let mut cave = vec![vec![true; cave_width]; 1];

    for mut rock in rock_generator(total_num_rocks) {
        rock.translate((2, (cave.len() + 3) as i32));
        while rock.bottom > cave.len() as i32 {
            rock.translate(get_direction(&mut wind, &rock));
        }

        let mut direction = get_direction(&mut wind, &rock);
        loop {
            if rock.shape.iter().any(|(x, y)| if *y >= cave.len() as i32 { false } else { cave[*y as usize][(x + direction.0) as usize] }) { direction.0 = 0; }
            if rock.shape.iter().any(|(x, y)| if *y + direction.1 >= cave.len() as i32 { false } else { cave[(y + direction.1) as usize][(x + direction.0) as usize] }) { direction.1 = 0; }
            rock.translate(direction);
            if direction.1 == 0 { break; }
            direction = get_direction(&mut wind, &rock);
        }

        while cave.len() < rock.bottom as usize + rock.height {
            cave.push(vec![false; cave_width]);
        }

        for (x, y) in &rock.shape {
            cave[*y as usize][*x as usize] = true;
        }
    }
    Ok((cave.len() - 1) as u32)
}

fn part02() -> Result<u64> {
    let input = std::fs::read_to_string("./data/day17.input")?
        .trim()
        .chars()
        .map(|char| match char {
            '<' => -1,
            '>' => 1,
            _ => unreachable!()
        })
        .collect::<Vec<i32>>();
        let mut wind = input
        .into_iter()
        .cycle();
    let mut height_after_rock = Vec::with_capacity(2022);
    let total_num_rocks = 100_000;
    let cave_width = 7;
    let mut cave: Vec<Vec<bool>> = Vec::new();

    for mut rock in rock_generator(total_num_rocks) {
        rock.translate((2, (cave.len() + 3) as i32));

        loop {
            let mut direction = get_direction(&mut wind, &rock);
            if rock.shape.iter().any(|(x, y)| if *y >= cave.len() as i32 { false } else { cave[*y as usize][(x + direction.0) as usize] }) { direction.0 = 0; }
            if rock.bottom == 0 || rock.shape.iter().any(|(x, y)| if *y + direction.1 >= cave.len() as i32 { false } else { cave[(y + direction.1) as usize][(x + direction.0) as usize] }) { direction.1 = 0; }
            rock.translate(direction);
            if direction.1 == 0 { break; }
        }

        while cave.len() < rock.bottom as usize + rock.height {
            cave.push(vec![false; cave_width]);
        }

        for (x, y) in &rock.shape {
            cave[*y as usize][*x as usize] = true;
        }

        height_after_rock.push(cave.len());
    }

    let mut periode = 0;
    let mut periode_height = 0;
    let checks = 6;
    let sample_max = 2_022;
    for num_rocks in 1..sample_max {
        match (2..checks)
            .map(|i| (height_after_rock[i * num_rocks] - height_after_rock[(i-1) * num_rocks]) as u64)
            .collect::<HashSet<u64>>()
            .into_iter()
            .at_most_one() {
                Ok(Some(h)) => {
                    periode_height = h;
                    periode = num_rocks;
                    break;
                },
                _ => ()
            }
    }
    let num_cycles = 1_000_000_000_000 as u64 / periode as u64;
    let remainder = (1_000_000_000_000 as u64 % periode as u64) as usize;

    Ok(num_cycles * periode_height + height_after_rock[remainder - 1] as u64)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
