use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
struct StatusCube {
    row: usize,
    col: usize,
    direction: Direction,
    section: Section,
}

impl StatusCube {
    fn make_move(&mut self, num_steps: usize, cube: &Cube) {
        for _ in 0..num_steps {
            let curr_section = &cube.faces[self.section];
            match self.direction {
                Direction::Right => {
                    if let Some(tile) = curr_section[self.row].get(self.col + 1) {
                        if *tile != Field::Wall {
                            self.col += 1;
                        }
                    } else {
                        let (new_section, new_row, new_col, new_direction) = cube.warp(self);
                        if cube.faces[new_section][new_row][new_col] != Field::Wall {
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.direction = new_direction;
                        }
                    }
                }
                Direction::Up => {
                    if self.row > 0 {
                        let tile_row = curr_section.get(self.row - 1).unwrap();
                        if tile_row[self.col] != Field::Wall {
                            self.row -= 1;
                        }
                    } else {
                        let (new_section, new_row, new_col, new_direction) = cube.warp(self);
                        if cube.faces[new_section][new_row][new_col] != Field::Wall {
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.direction = new_direction;
                        }
                    }
                }
                Direction::Left => {
                    if self.col > 0 {
                        let tile = curr_section[self.row].get(self.col - 1).unwrap();
                        if *tile != Field::Wall {
                            self.col -= 1;
                        }
                    } else {
                        let (new_section, new_row, new_col, new_direction) = cube.warp(self);
                        if cube.faces[new_section][new_row][new_col] != Field::Wall {
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.direction = new_direction;
                        }
                    }
                }
                Direction::Down => {
                    if let Some(tile_row) = curr_section.get(self.row + 1) {
                        if tile_row[self.col] != Field::Wall {
                            self.row += 1;
                        }
                    } else {
                        let (new_section, new_row, new_col, new_direction) = cube.warp(self);
                        if cube.faces[new_section][new_row][new_col] != Field::Wall {
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.direction = new_direction;
                        }
                    }
                }
            }
        }
    }
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
}

#[derive(Debug, Clone, Copy)]
pub enum Section {
    Section1,
    Section2,
    Section3,
    Section4,
    Section5,
    Section6,
}

impl<T> Index<Section> for Vec<T> {
    type Output = T;

    fn index(&self, index: Section) -> &Self::Output {
        &self[index as usize]
    }
}
impl<T> IndexMut<Section> for Vec<T> {
    fn index_mut(&mut self, index: Section) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

struct Cube {
    pub faces: Vec<Vec<Vec<Field>>>,
    pub width: usize,
    pub height: usize,
}

impl Cube {
    fn get_row_for_section(&self, section: &Section) -> usize {
        match section {
            Section::Section1 => 0,
            Section::Section2 => 0,
            Section::Section3 => self.height,
            Section::Section4 => self.height * 2,
            Section::Section5 => self.height * 2,
            Section::Section6 => self.height * 3,
        }
    }
    fn get_col_for_section(&self, section: &Section) -> usize {
        match section {
            Section::Section1 => self.width,
            Section::Section2 => self.width * 2,
            Section::Section3 => self.width,
            Section::Section4 => 0,
            Section::Section5 => self.width,
            Section::Section6 => 0,
        }
    }
    fn from_map(map: &Vec<Vec<Field>>) -> Self {
        let height = map.len() / 4;
        let width = map[0].len() / 3;
        let mut cube = Self {
            faces: Default::default(),
            width,
            height,
        };
        // Section 1
        cube.faces.push(
            map[0..height]
                .iter()
                .map(|i| i[width..width * 2].iter().copied().collect_vec())
                .collect_vec(),
        );
        // Section 2
        cube.faces.push(
            map[0..height]
                .iter()
                .map(|i| i[width * 2..width * 3].iter().copied().collect_vec())
                .collect_vec(),
        );
        // Section 3
        cube.faces.push(
            map[height..height * 2]
                .iter()
                .map(|i| i[width..width * 2].iter().copied().collect_vec())
                .collect_vec(),
        );
        // Section 4
        cube.faces.push(
            map[height * 2..height * 3]
                .iter()
                .map(|i| i[0..width].iter().copied().collect_vec())
                .collect_vec(),
        );
        // Section 5
        cube.faces.push(
            map[height * 2..height * 3]
                .iter()
                .map(|i| i[width..width * 2].iter().copied().collect_vec())
                .collect_vec(),
        );
        // Section 6
        cube.faces.push(
            map[height * 3..height * 4]
                .iter()
                .map(|i| i[0..width].iter().copied().collect_vec())
                .collect_vec(),
        );
        cube
    }
    fn warp(&self, status: &StatusCube) -> (Section, usize, usize, Direction) {
        match (status.section, status.direction) {
            (Section::Section1, Direction::Right) => {
                (Section::Section2, status.row, 0, Direction::Right)
            }
            (Section::Section1, Direction::Up) => {
                (Section::Section6, status.col, 0, Direction::Right)
            }
            (Section::Section1, Direction::Left) => (
                Section::Section4,
                self.faces[Section::Section4].len() - 1 - status.row,
                0,
                Direction::Right,
            ),
            (Section::Section1, Direction::Down) => {
                (Section::Section3, 0, status.col, Direction::Down)
            }
            (Section::Section2, Direction::Right) => (
                Section::Section5,
                self.faces[Section::Section5].len() - 1 - status.row,
                self.faces[Section::Section5][0].len() - 1,
                Direction::Left,
            ),
            (Section::Section2, Direction::Up) => (
                Section::Section6,
                self.faces[Section::Section6].len() - 1,
                status.col,
                Direction::Up,
            ),
            (Section::Section2, Direction::Left) => (
                Section::Section1,
                status.row,
                self.faces[Section::Section1][0].len() - 1,
                Direction::Left,
            ),
            (Section::Section2, Direction::Down) => (
                Section::Section3,
                status.col,
                self.faces[Section::Section3][0].len() - 1,
                Direction::Left,
            ),
            (Section::Section3, Direction::Right) => (
                Section::Section2,
                self.faces[Section::Section3].len() - 1,
                status.row,
                Direction::Up,
            ),
            (Section::Section3, Direction::Up) => (
                Section::Section1,
                self.faces[Section::Section1].len() - 1,
                status.col,
                Direction::Up,
            ),
            (Section::Section3, Direction::Left) => {
                (Section::Section4, 0, status.row, Direction::Down)
            }
            (Section::Section3, Direction::Down) => {
                (Section::Section5, 0, status.col, Direction::Down)
            }
            (Section::Section4, Direction::Right) => {
                (Section::Section5, status.row, 0, Direction::Right)
            }
            (Section::Section4, Direction::Up) => {
                (Section::Section3, status.col, 0, Direction::Right)
            }
            (Section::Section4, Direction::Left) => (
                Section::Section1,
                self.faces[Section::Section1].len() - 1 - status.row,
                0,
                Direction::Right,
            ),
            (Section::Section4, Direction::Down) => {
                (Section::Section6, 0, status.col, Direction::Down)
            }
            (Section::Section5, Direction::Right) => (
                Section::Section2,
                self.faces[Section::Section2].len() - 1 - status.row,
                self.faces[Section::Section2][0].len() - 1,
                Direction::Left,
            ),
            (Section::Section5, Direction::Up) => (
                Section::Section3,
                self.faces[Section::Section3].len() - 1,
                status.col,
                Direction::Up,
            ),
            (Section::Section5, Direction::Left) => (
                Section::Section4,
                status.row,
                self.faces[Section::Section4][0].len() - 1,
                Direction::Left,
            ),
            (Section::Section5, Direction::Down) => (
                Section::Section6,
                status.col,
                self.faces[Section::Section6][0].len() - 1,
                Direction::Left,
            ),
            (Section::Section6, Direction::Right) => (
                Section::Section5,
                self.faces[Section::Section5].len() - 1,
                status.row,
                Direction::Up,
            ),
            (Section::Section6, Direction::Up) => (
                Section::Section4,
                self.faces[Section::Section4].len() - 1,
                status.col,
                Direction::Up,
            ),
            (Section::Section6, Direction::Left) => {
                (Section::Section1, 0, status.row, Direction::Down)
            }
            (Section::Section6, Direction::Down) => {
                (Section::Section2, 0, status.col, Direction::Down)
            }
        }
    }
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
            if let Some(new_col) = self.get_next_col(curr_it) {
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
            if let Some(new_row) = self.get_next_row(curr_it) {
                self.row = new_row;
            } else {
                return;
            }
        }
    }

    fn get_next_col<'a, I>(&self, curr_it: &mut I) -> Option<usize>
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

    fn get_next_row<'a, I>(&self, curr_it: &mut I) -> Option<usize>
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

fn part02(cube: &Cube, instructions: &Vec<String>) -> Result<u32> {
    let mut status = StatusCube {
        section: Section::Section1,
        row: 0,
        col: 0,
        direction: Direction::Right,
    };
    for ins in instructions {
        match ins.as_str() {
            "R" => status.turn_clockwise(),
            "L" => status.turn_counter_clockwise(),
            n => status.make_move(n.parse().unwrap(), cube),
        }
    }
    let password = 1_000 * (status.row + 1 + cube.get_row_for_section(&status.section))
        + 4 * (status.col + 1 + cube.get_col_for_section(&status.section))
        + match status.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
    Ok(password as u32)
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
    let cube = Cube::from_map(&map);
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
    println!("{:?}", part02(&cube, &parsed_instructions).unwrap());
    Ok(())
}
