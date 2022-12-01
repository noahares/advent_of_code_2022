use anyhow::Result;

fn main() -> Result<()> {
    println!("{:?}", std::fs::read_to_string("./data/day01.example")?
        .trim()
        .split("\n\n")
        .map(|block| block.split("\n").map(|item| item.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>()
        .iter()
        .max()
        .unwrap());
    Ok(())
}
