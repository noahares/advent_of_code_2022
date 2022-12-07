use anyhow::Result;
use std::collections::HashSet;

fn part01() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day06.input")?;
    for i in 4..input.len() {
        let mut set: HashSet<char> = HashSet::new();
        if input[i-4..i].chars().all(move |x| set.insert(x)) {
            return Ok(i as u32)
        }
    }
    Ok(input.len() as u32)
}

fn part02() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day06.input")?;
    for i in 14..input.len() {
        let mut set: HashSet<char> = HashSet::new();
        if input[i-14..i].chars().all(move |x| set.insert(x)) {
            return Ok(i as u32)
        }
    }
    Ok(input.len() as u32)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
