use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    std::fs::read_to_string("./data/day02.input")?
        .trim()
        .split("\n")
        .map(|block| block.split_once(" ").iter().collect_tuple().map(|x| match x {
            (a, b) if ((a == "A" && b == "X") ||
                       (a == "B" && b == "Y") ||
                       (a == "C" && b == "Z")) => 3,
            (_, _) => 0,
        }).unwrap().sum())
        .collect::<Vec<i32>>();

    Ok(())
}
