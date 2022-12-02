use anyhow::Result;

fn main() -> Result<()> {
    let result = std::fs::read_to_string("./data/day02.input")?
        .trim()
        .split("\n")
        .map(|block| block.split_once(" ").unwrap()).map(|x| match x {
            ("A", "X") => 4,
            ("A", "Y") => 1,
            ("A", "Z") => 7,
            ("B", "X") => 8,
            ("B", "Y") => 5,
            ("B", "Z") => 2,
            ("C", "X") => 3,
            ("C", "Y") => 9,
            ("C", "Z") => 6,
            (_, _) => 0,
        }).sum::<i32>();
    println!("{}", result);
    Ok(())
}
