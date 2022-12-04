use anyhow::Result;
use itertools::Itertools;

fn part01() -> Result<u32> {
    Ok(std::fs::read_to_string("./data/day04.input")?
        .trim()
        .split(|c: char| !c.is_numeric())
        .map(|s| s.parse::<u32>().unwrap())
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let bounds: Vec<u32> = chunk.collect();
            (bounds[0] <= bounds[2] && bounds[1] >= bounds[3] ||
            bounds[2] <= bounds[0] && bounds[3] >= bounds[1]) as u32

        })
        .collect_vec()
        .into_iter()
        .sum())
}

fn part02() -> Result<u32> {
    Ok(std::fs::read_to_string("./data/day04.input")?
        .trim()
        .split(|c: char| !c.is_numeric())
        .map(|s| s.parse::<u32>().unwrap())
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let bounds: Vec<u32> = chunk.collect();
            (bounds[0] <= bounds[3] && bounds[1] >= bounds[2]) as u32

        })
        .collect_vec()
        .into_iter()
        .sum())
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
