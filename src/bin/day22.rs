use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    Tile,
    Wall,
    Void,
}

#[derive(Debug)]
struct Status {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Status {
    fn turn_clockwise(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_counter_clockwise(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn move_horizontally<'a, I>(&mut self, num_steps: usize, curr_it: &mut I)
    where
        I: Iterator<Item = (usize, &'a Field)>,
    {
        for _ in 0..num_steps {
            if let Some(new_col) = self.get_next_row(curr_it) {
                self.col = new_col;
            } else {
                return;
            }
        }
    }

    fn move_vertically<'a, I>(&mut self, num_steps: usize, curr_it: &mut I)
    where
        I: Iterator<Item = (usize, &'a Vec<Field>)>,
    {
        for _ in 0..num_steps {
            if let Some(new_row) = self.get_next_col(curr_it) {
                self.row = new_row;
            } else {
                return;
            }
        }
    }

    fn get_next_row<'a, I>(&self, curr_it: &mut I) -> Option<usize>
    where
        I: Iterator<Item = (usize, &'a Field)>,
    {
        let next_tile = curr_it.next().unwrap();
        if *next_tile.1 == Field::Wall {
            return None;
        }
        if *next_tile.1 == Field::Tile {
            return Some(next_tile.0);
        }
        // tile is void
        loop {
            let next_tile = curr_it.next().unwrap();
            if *next_tile.1 == Field::Wall {
                return None;
            }
            if *next_tile.1 == Field::Tile {
                return Some(next_tile.0);
            }
        }
    }

    fn get_next_col<'a, I>(&self, curr_it: &mut I) -> Option<usize>
    where
        I: Iterator<Item = (usize, &'a Vec<Field>)>,
    {
        let next_tile = curr_it.next().unwrap();
        if next_tile.1[self.col] == Field::Wall {
            return None;
        }
        if next_tile.1[self.col] == Field::Tile {
            return Some(next_tile.0);
        }
        // tile is void
        loop {
            let next_tile = curr_it.next().unwrap();
            if next_tile.1[self.col] == Field::Wall {
                return None;
            }
            if next_tile.1[self.col] == Field::Tile {
                return Some(next_tile.0);
            }
        }
    }
    fn make_move(&mut self, map: &Vec<Vec<Field>>, num_steps: usize) {
        match self.direction {
            Direction::Up => {
                let mut curr_it = map.iter().enumerate().rev().cycle();
                for _ in self.row..map.len() {
                    curr_it.next();
                }
                self.move_vertically(num_steps, &mut curr_it);
            }
            Direction::Down => {
                let mut curr_it = map.iter().enumerate().cycle();
                for _ in 0..=self.row {
                    curr_it.next();
                }
                self.move_vertically(num_steps, &mut curr_it);
            }
            Direction::Left => {
                let mut curr_it = map[self.row].iter().enumerate().rev().cycle();
                for _ in self.col..map[self.row].len() {
                    curr_it.next();
                }
                self.move_horizontally(num_steps, &mut curr_it);
            }
            Direction::Right => {
                let mut curr_it = map[self.row].iter().enumerate().cycle();
                for _ in 0..=self.col {
                    curr_it.next();
                }
                self.move_horizontally(num_steps, &mut curr_it);
            }
        }
    }
}

fn part01(map: &Vec<Vec<Field>>, instructions: &Vec<String>) -> Result<u32> {
    let mut status = Status {
        row: 0,
        col: map[0]
            .iter()
            .enumerate()
            .find(|(_, tile)| **tile == Field::Tile)
            .unwrap()
            .0,
        direction: Direction::Right,
    };
    for ins in instructions {
        match ins.as_str() {
            "R" => status.turn_clockwise(),
            "L" => status.turn_counter_clockwise(),
            n => status.make_move(map, n.parse().unwrap()),
        }
    }
    let password = 1_000 * (status.row + 1)
        + 4 * (status.col + 1)
        + match status.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
    Ok(password as u32)
}

fn part02() -> Result<u32> {
    Ok(0)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/day22.input")?;
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Field::Tile,
                    '#' => Field::Wall,
                    ' ' => Field::Void,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let max_col = map.iter().map(|v| v.len()).max().unwrap();
    map.iter_mut().for_each(|v| {
        if v.len() < max_col {
            v.extend(vec![Field::Void; max_col - v.len()].iter())
        }
    });
    let regex = Regex::new(r"\d+").unwrap();
    let mut parsed_instructions = Vec::new();
    let mut last_end = 0;
    let instructions = instructions.trim();
    for captures in regex.captures_iter(instructions) {
        let start = captures.get(0).unwrap().start();
        if start > last_end {
            parsed_instructions.push(instructions[last_end..start].to_string());
        }
        parsed_instructions.push(captures[0].to_string());
        last_end = captures.get(0).unwrap().end();
    }
    if last_end < instructions.len() {
        parsed_instructions.push(instructions[last_end..].to_string());
    }
    println!("{:?}", part01(&map, &parsed_instructions).unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
