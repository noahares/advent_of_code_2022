use anyhow::Result;
use std::str::FromStr;
use itertools::Itertools;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
       .lines()
       .filter_map(|line| line.parse::<T>().ok())
       .collect())
}

pub fn read_vec_per_line<T, F>(path: &str, f: F) -> Result<Vec<Vec<T>>>
where
T: FromStr,
F: Fn(char) -> Option<T>,
{
    Ok(std::fs::read_to_string(path)?
       .lines()
       .map(|line| line.chars().map(|c| f(c).unwrap()).collect())
       .collect())
}

pub fn read_one_line<T>(path: &str, sep: &str) -> Result<Vec<T>>
where
T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
       .trim()
       .split(sep)
       .filter_map(|c| c.parse::<T>().ok())
       .collect())
}

pub fn read_lines_sperated_by_empty_line<'a, T>(path: &'a str) -> Result<Vec<Vec<T>>>
where
T: FromStr, Vec<T>: FromIterator<&'a str>
{
    let mut result_split_by_empty_line = Vec::new();
    let mut item = Vec::new();
    for line in std::fs::read_to_string(path)?.lines() {
        let it = line.trim();
        if it.is_empty() {
            result_split_by_empty_line.push(item);
            item = Vec::new();
        } else {
            item.push(it);
        }
    }
    if !item.is_empty() {
        result_split_by_empty_line.push(item);
    }
    Ok(result_split_by_empty_line)
}
