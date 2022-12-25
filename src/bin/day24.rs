use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Blizzard {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Debug)]
struct Basin {
    width: i32,
    height: i32,
    start: (i32, i32),
    end: (i32, i32),
    walls: HashSet<(i32, i32)>,
    blizzards: Vec<Blizzard>,
    map_cache: HashMap<i32, HashSet<(i32, i32)>>,
}

impl Basin {
    fn find_shortest_path_time(
        &mut self,
        start: (i32, i32),
        end: (i32, i32),
        start_time: i32,
    ) -> i32 {
        let mut visited: HashSet<(i32, (i32, i32))> = HashSet::new();
        let mut queue = VecDeque::from([(start_time, start)]);
        while !queue.is_empty() {
            let (mut time, pos) = queue.pop_front().unwrap();
            time += 1;
            let map = self.map_at_time(time);
            for next_cell in self.neighbors(pos, &map) {
                if !visited.contains(&(time, next_cell)) {
                    if next_cell == end {
                        return time;
                    }
                    visited.insert((time, next_cell));
                    queue.push_back((time, next_cell))
                }
            }
        }
        -1
    }

    fn map_at_time(&mut self, time: i32) -> HashSet<(i32, i32)> {
        let basin_width = self.width - 2;
        let basin_height = self.height - 2;
        let t = time % (basin_width * basin_height);
        if let Some(cached_points) = self.map_cache.get(&t) {
            return cached_points.clone();
        }

        let mut points: HashSet<(i32, i32)> = self.walls.clone();
        for blizzard in &self.blizzards {
            let x = positive_modulo(
                blizzard.x - 1 + blizzard.direction.value().0 * time,
                basin_height,
            ) + 1;
            let y = positive_modulo(
                blizzard.y - 1 + blizzard.direction.value().1 * time,
                basin_width,
            ) + 1;
            points.insert((x, y));
        }
        self.map_cache.insert(t, points.clone());
        points
    }

    fn neighbors(&self, pos: (i32, i32), map: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut neighbors: Vec<(i32, i32)> = Vec::new();
        for (dx, dy) in [(1, 0), (0, 1), (0, -1), (-1, 0), (0, 0)] {
            let x = pos.0 + dx;
            let y = pos.1 + dy;
            if x < 0 || x >= self.height || y < 0 || y >= self.width {
                continue;
            }
            let p = (x, y);
            if !map.contains(&p) {
                neighbors.push(p)
            }
        }
        neighbors
    }
}

fn positive_modulo(x: i32, y: i32) -> i32 {
    ((x % y) + y) % y
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("./data/day24.input")?
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let start_pos = (0, 1);
    let num_rows = input.len() as i32;
    let num_cols = input.first().unwrap().len() as i32;
    let end_pos = (num_rows - 1, num_cols - 2);
    let blizzards: Vec<Blizzard> = input
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.char_indices().filter_map(move |(y, c)| match c {
                '>' => Some(Blizzard {
                    x: x as i32,
                    y: y as i32,
                    direction: Direction::Right,
                }),
                '<' => Some(Blizzard {
                    x: x as i32,
                    y: y as i32,
                    direction: Direction::Left,
                }),
                'v' => Some(Blizzard {
                    x: x as i32,
                    y: y as i32,
                    direction: Direction::Down,
                }),
                '^' => Some(Blizzard {
                    x: x as i32,
                    y: y as i32,
                    direction: Direction::Up,
                }),
                _ => None,
            })
        })
        .collect();
    let mut walls = HashSet::new();
    for x in 0..num_cols {
        if x != start_pos.1 {
            walls.insert((0, x));
        }
        if x != end_pos.1 {
            walls.insert((num_rows - 1, x));
        }
    }
    for y in 0..num_rows {
        walls.insert((y, 0));
        walls.insert((y, num_cols - 1));
    }
    let mut basin = Basin {
        width: num_cols as i32,
        height: num_rows as i32,
        start: start_pos,
        end: end_pos,
        walls,
        blizzards,
        map_cache: HashMap::new(),
    };
    let time = basin.find_shortest_path_time(basin.start, basin.end, 0);
    println!("part1 {}", time);
    let time = basin.find_shortest_path_time(basin.end, basin.start, time);
    let time = basin.find_shortest_path_time(basin.start, basin.end, time);
    println!("part2 {}", time);
    Ok(())
}
