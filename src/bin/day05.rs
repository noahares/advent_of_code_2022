use anyhow::Result;

fn part01() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day05.example")?
        .trim()
        .to_string();
    let (drawing, moves) = input.split_once("\n\n").unwrap();
    let number_of_piles = drawing.trim().chars().last().unwrap().to_digit(10).unwrap();
    Ok(number_of_piles)
}

fn part02() -> Result<u32> {
    Ok(0)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
