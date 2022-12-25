use anyhow::Result;

fn to_snafu(mut n: i64) -> String {
    let mut digits: Vec<i8> = Vec::with_capacity(20);
    while n > 0 {
        let i = (n % 5) as i8;
        n /= 5;
        digits.push(i);
    }
    for i in 0..digits.len() {
        if digits[i] > 2 {
            let offset = digits[i] - 5;
            digits[i+1] += 1;
            digits[i] = offset;
        }
    }
    digits.iter().rev().map(|d| match *d {
        -2 => "=".to_string(),
        -1 => "-".to_string(),
        n => n.to_string()
    })
    .collect::<String>()

}

fn part01() -> Result<String> {
    let sum: i64 = std::fs::read_to_string("./data/day25.input")?
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| match c {
            '=' => -2,
            '-' => -1,
            n => n.to_digit(10).unwrap() as i64
        }).rev().enumerate()
             .map(|(pos, digit)| 5_i64.pow(pos as u32) * digit).sum::<i64>()
            )
    .sum();
    let result = to_snafu(sum);
    Ok(result)
}

fn part02() -> Result<u32> {
    Ok(0)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
