use anyhow::Result;

fn main() -> Result<()> {

    // part 1
    let result = std::fs::read_to_string("./data/day02.input")?
        .trim()
        .split("\n")
        .map(|block| block.as_bytes())
        .map(|x| ((x[2] - x[0] + 2) % 3 * 3 + x[2] - 87) as i32)
        .sum::<i32>();
    println!("{}", result);

    // part 2
    let result2 = std::fs::read_to_string("./data/day02.input")?
        .trim()
        .split("\n")
        .map(|block| block.as_bytes())
        .map(|x| ((x[2] - 88) * 3 + (x[2] + x[0] + 2) % 3 + 1) as i32)
        .sum::<i32>();
    println!("{}", result2);
    Ok(())
}
