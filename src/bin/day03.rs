use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

fn part01() -> Result<u32> {
    Ok(std::fs::read_to_string("./data/day03.input")?
        .trim()
        .split("\n")
        .map(|block| block.as_bytes().chunks(block.len() / 2))
        .flatten()
        .map(|block| -> HashSet<&u8> {HashSet::from_iter(block.iter())})
        .chunks(2)
        .into_iter()
        .map(|mut sets| {
            let intersection: &u8 = sets.next()
                .map(|set: HashSet<&u8, _>| sets.fold(set, |set1: HashSet<&u8, _>, set2: HashSet<&u8, _>| &set1 & &set2)).unwrap().iter().next().unwrap();
            let result = ((intersection.to_ascii_lowercase() - 96) + (26 * intersection.is_ascii_uppercase() as u8)) as u32;
            result as u32
        })
        .collect_vec()
        .into_iter()
        .sum())
}

fn part02() -> Result<u32> {
    Ok(std::fs::read_to_string("./data/day03.input")?
        .trim()
        .split("\n")
        .map(|block| block.as_bytes())
        .map(|block| -> HashSet<&u8> {HashSet::from_iter(block.iter())})
        .chunks(3)
        .into_iter()
        .map(|mut sets| {
            let intersection: &u8 = sets.next()
                .map(|set: HashSet<&u8, _>| sets.fold(set, |set1: HashSet<&u8, _>, set2: HashSet<&u8, _>| &set1 & &set2)).unwrap().iter().next().unwrap();
            let result = ((intersection.to_ascii_lowercase() - 96) + (26 * intersection.is_ascii_uppercase() as u8)) as u32;
            result as u32
        })
        .collect_vec()
        .into_iter()
        .sum::<u32>())
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
