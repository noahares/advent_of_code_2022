use anyhow::Result;

fn main() -> Result<()> {
    let mut sums = std::fs::read_to_string("./data/day01.input")?
        .trim()
        .split("\n\n")
        .map(|block| block.split("\n").map(|item| item.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    sums.sort_unstable_by(|a, b| b.cmp(a));

    // part 1
    println!("{:?}", sums
        .iter()
        .take(1)
        .sum::<i32>());

    // part 2
    println!("{:?}", sums
        .iter()
        .take(3)
        .sum::<i32>());
    Ok(())
}
