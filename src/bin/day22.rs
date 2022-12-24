use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        }
    }

    fn turn_clockwise(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }

    fn turn_counter_clockwise(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FieldType {
    Tile,
    Wall,
    Void
}

#[derive(Debug)]
struct Field {
   pos: (usize, usize),
   field_type: FieldType
}

struct Wormholes {
    left: Vec<usize>,
    up: Vec<usize>,
    down: Vec<usize>,
}

impl Wormholes {
    fn new(map: &Vec<Vec<Field>>) -> Self {
        let wormholes_left = map.iter().map(|line| line.iter().find_position(|&field| field.field_type != FieldType::Void).unwrap().0).collect_vec();
        let longest_row = map.iter().max_by(|&a, &b| a.len().cmp(&b.len())).unwrap().len();
        let mut wormholes_up = vec![0; longest_row];
        let mut wormholes_down = vec![map.len() - 1; longest_row];
        for row in 1..map.len()-1 {
            (0..map[row].len()).for_each(|column| {
                if map[row-1].len() >= map[row].len() && map[row-1][column].field_type == FieldType::Void && map[row][column].field_type != FieldType::Void
                || wormholes_up[column] == 0 && (map[row-1].len() - 1) < column {
                    wormholes_up[column] = row;
                }
                if (map[row+1].len() <= column || map[row+1][column].field_type == FieldType::Void) && map[row][column].field_type != FieldType::Void {
                    wormholes_down[column] = row;
                }
            });
        }
        Self { left: wormholes_left, up: wormholes_up, down: wormholes_down }
    }
}

impl Field {
    fn get_new_position<'a>(&self, map: &'a Vec<Vec<Field>>, direction: &Direction, wormholes: &Wormholes) -> Option<&'a Field> {
        let mods = (wormholes.down[self.pos.1] as i32 + 1, (map[self.pos.0].len()) as i32);
        let mut neigbor = (((self.pos.0 as i32 + direction.value().0 + mods.0) % mods.0) as usize, ((self.pos.1 as i32 + direction.value().1 + mods.1) % mods.1) as usize);
        neigbor.0 = neigbor.0.clamp(wormholes.up[neigbor.1], wormholes.down[neigbor.1]);
        neigbor.1 = neigbor.1.clamp(wormholes.left[neigbor.0], mods.1 as usize);
        if map[neigbor.0][neigbor.1].field_type == FieldType::Tile { Some(&map[neigbor.0][neigbor.1]) }
        else { None }
    }
}

fn part01() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day22.input")?;
    let (map, instructions) = input
        .split_once("\n\n").unwrap();
    let map = map
        .lines()
        .enumerate()
        .map(|(i, line)| line.char_indices().map(move |(j, c)| Field { pos: (i, j), field_type: match c {
            '.' => FieldType::Tile,
            '#' => FieldType::Wall,
            ' ' => FieldType::Void,
            _ => unreachable!()

        }}).collect_vec())
    .collect_vec();
    let wormholes = Wormholes::new(&map);
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
    // for line in &map {
    //     println!("{line:?}");
    // }
    // println!("{parsed_instructions:?}");
    // println!("{:?}", wormholes.up);
    // println!("{:?}", wormholes.down);
    // println!("{:?}", wormholes.left);
    let mut my_position = &map[0][wormholes.left[0]];
    let mut my_direction = Direction::Right;
    for ins in &parsed_instructions {
        // println!("{:?}", my_direction);
        // println!("{:?}", ins);
        match ins.as_str() {
            "R" => my_direction = my_direction.turn_clockwise(),
            "L" => my_direction = my_direction.turn_counter_clockwise(),
            n => {
                let num_steps: usize = n.parse().unwrap();
                for _ in 0..num_steps {
                    match my_position.get_new_position(&map, &my_direction, &wormholes) {
                        Some(pos) => my_position = pos,
                        None => break
                    }
                }
                // println!("{:?}", my_position);
            }
        }

    }
    // println!("{:?}", my_position);
    let password = 1_000 * (my_position.pos.0 + 1) + 4 * (my_position.pos.1 + 1) + match my_direction {
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
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
