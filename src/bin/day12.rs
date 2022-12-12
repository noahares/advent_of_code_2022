use anyhow::Result;
use itertools::Itertools;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Item {
    distance: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn position_to_index((a, b): (usize, usize), dim: usize) -> usize {
    a * dim + b
}

fn generate_neighbors((a, b): (usize, usize), dim: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if a + 1 < dim.0 { neighbors.push(( a + 1, b)); }
    if a as i32 - 1 >= 0 { neighbors.push((a - 1, b)); }
    if b + 1 < dim.1 { neighbors.push((a, b + 1)); }
    if b as i32 - 1 >= 0 { neighbors.push((a, b - 1)); }
    neighbors
}

fn part01() -> Result<u32> {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut i: i32 = -1;
    let mut j: i32 = -1;
    let input: Vec<Vec<u32>> = std::fs::read_to_string("./data/day12.input")?
        .lines()
        .map(|line| {
            i += 1;
            j = -1;
            line.bytes().map(|byte| {
                j += 1;
                match byte {
                    83 => {
                        start = (i as usize, j as usize);
                        'a' as u32
                    },
                    69 => {
                        end = (i as usize, j as usize);
                        'z' as u32
                    }
                    _ => byte as u32
                }
            }).collect_vec()
        })
        .collect();
    let dim = (input.len(), input[0].len());
    let mut visited = vec![false; dim.0 * dim.1];
    let mut dist = vec![usize::MAX; dim.0 * dim.1];
    let mut pq = BinaryHeap::new();
    dist[position_to_index(start, dim.1)] = 0;
    visited[position_to_index(start, dim.1)] = true;
    pq.push(Item {distance: 0, position: start});

    while let Some(Item {distance, position}) = pq.pop() {
        if position == end { return Ok(distance as u32); }
        if distance > dist[position_to_index(position, dim.1)] { continue; }
        let val_of_current = input[position.0][position.1];
        visited[position_to_index(position, dim.1)] = true;

        for neighbor in generate_neighbors(position, dim) {
            let val_of_neighbor = input[neighbor.0][neighbor.1];
            if val_of_neighbor > val_of_current + 1 { continue; }
            if !visited[position_to_index(neighbor, dim.1)] && distance + 1 < dist[position_to_index(neighbor, dim.1)] {
                pq.push(Item { distance: distance + 1, position: neighbor });
                dist[position_to_index(neighbor, dim.1)] = distance + 1;
            }
        }
    }
    Ok(0)
}

fn part02() -> Result<u32> {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut end: (usize, usize) = (0, 0);
    let mut i: i32 = -1;
    let mut j: i32 = -1;
    let input: Vec<Vec<u32>> = std::fs::read_to_string("./data/day12.input")?
        .lines()
        .map(|line| {
            i += 1;
            j = -1;
            line.bytes().map(|byte| {
                j += 1;
                match byte {
                    83 | 97 => {
                        starts.push((i as usize, j as usize));
                        'a' as u32
                    },
                    69 => {
                        end = (i as usize, j as usize);
                        'z' as u32
                    }
                    _ => byte as u32
                }
            }).collect_vec()
        })
        .collect();
    let dim = (input.len(), input[0].len());
    let mut visited = vec![false; dim.0 * dim.1];
    let mut dist = vec![usize::MAX; dim.0 * dim.1];
    let mut pq = BinaryHeap::new();
    for start in starts {
        dist[position_to_index(start, dim.1)] = 0;
        visited[position_to_index(start, dim.1)] = true;
        pq.push(Item {distance: 0, position: start});
    }

    while let Some(Item {distance, position}) = pq.pop() {
        if position == end { return Ok(distance as u32); }
        if distance > dist[position_to_index(position, dim.1)] { continue; }
        let val_of_current = input[position.0][position.1];
        visited[position_to_index(position, dim.1)] = true;

        for neighbor in generate_neighbors(position, dim) {
            let val_of_neighbor = input[neighbor.0][neighbor.1];
            if val_of_neighbor > val_of_current + 1 { continue; }
            if !visited[position_to_index(neighbor, dim.1)] && distance + 1 < dist[position_to_index(neighbor, dim.1)] {
                pq.push(Item { distance: distance + 1, position: neighbor });
                dist[position_to_index(neighbor, dim.1)] = distance + 1;
            }
        }
    }
    Ok(0)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
