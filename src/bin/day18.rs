use std::collections::HashSet;

use anyhow::Result;

fn generate_neighbors((a, b, c): (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    let mut neighbors: Vec<(i32, i32, i32)> = Vec::new();
    let adj = vec![-1, 1];
    for i in &adj {
        neighbors.push((a + i, b,  c));
        neighbors.push((a, b + i,  c));
        neighbors.push((a , b,  c + i));
    }
    neighbors.into_iter()
}

fn part01() -> Result<u32> {
    let qubes: HashSet<(i32, i32, i32)> = std::fs::read_to_string("./data/day18.input")?
        .trim()
        .lines()
        .map(|line| sscanf::scanf!(line,
                                   "{i32},{i32},{i32}").unwrap())
        .collect();

    let possible_surface_contribution = 6;
    let surface_area = qubes.iter().fold(0, |acc, qube| acc + possible_surface_contribution - generate_neighbors(*qube).filter(|neighbor| qubes.contains(neighbor)).count());
    Ok(surface_area as u32)
}

fn part02() -> Result<u32> {
    let qubes: HashSet<(i32, i32, i32)> = std::fs::read_to_string("./data/day18.input")?
        .trim()
        .lines()
        .map(|line| sscanf::scanf!(line,
                                   "{i32},{i32},{i32}").unwrap())
        .collect();

    let x_dim = qubes.iter().max_by(|q, p| q.0.cmp(&p.0)).unwrap().0 + 1;
    let y_dim = qubes.iter().max_by(|q, p| q.1.cmp(&p.1)).unwrap().1 + 1;
    let z_dim = qubes.iter().max_by(|q, p| q.2.cmp(&p.2)).unwrap().2 + 1;

    let mut surface_area = 0;

    let mut visited = HashSet::new();
    let mut next = HashSet::new();
    next.insert((-1, -1, -1));
    while let Some(qube) = next.iter().next().cloned() {
        next.remove(&qube);
        for neighbor in generate_neighbors(qube) {
            if !((-1..=x_dim).contains(&neighbor.0) && (-1..=y_dim).contains(&neighbor.1) && (-1..=z_dim).contains(&neighbor.2)) { continue; }
            if qubes.contains(&neighbor) {
                surface_area += 1;
            } else if !visited.contains(&neighbor) {
                next.insert(neighbor);
            }
            visited.insert(neighbor);
        }
    }

    Ok(surface_area as u32)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
