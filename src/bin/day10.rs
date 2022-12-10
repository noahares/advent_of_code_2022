use anyhow::Result;

fn part01() -> Result<i32> {
    let mut num_cycles = 0;
    let mut sum_of_signal_strength = 0;
    let mut x = 1;
    std::fs::read_to_string("./data/day10.input")?
        .trim()
        .lines()
        .for_each(|line| match line.split_once(" ") {
            Some((_, number)) => {
                for _ in 0..2 {
                    num_cycles += 1;
                    if (num_cycles - 20) % 40 == 0 {
                        sum_of_signal_strength += num_cycles * x;
                    }
                }
                x += number.parse::<i32>().unwrap();
            },
            None => {
                num_cycles += 1;
                if (num_cycles - 20) % 40 == 0 {
                    sum_of_signal_strength += num_cycles * x;
                }
            }
        });
    Ok(sum_of_signal_strength)
}

fn part02() -> Result<()> {
    let mut num_cycles = 0;
    let mut ctr: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let mut x = 1;
    let mut row = -1;
    std::fs::read_to_string("./data/day10.input")?
        .trim()
        .lines()
        .for_each(|line| match line.split_once(" ") {
            Some((_, number)) => {
                for _ in 0..2 {
                    num_cycles += 1;
                    if num_cycles % 40 == 1 {
                        row += 1;
                    }
                    let ctr_col = (num_cycles - 1) % 40;
                    println!("{}", ctr_col);
                    if x == ctr_col || x - 1 == ctr_col || x + 1 == ctr_col {
                        ctr[row as usize][ctr_col as usize] = '#';
                    }
                }
                x += number.parse::<i32>().unwrap();
            },
            None => {
                num_cycles += 1;
                if num_cycles % 40 == 1 {
                    row += 1;
                }
                let ctr_col = (num_cycles - 1) % 40;
                println!("{}", ctr_col);
                if x == ctr_col || x - 1 == ctr_col || x + 1 == ctr_col {
                    ctr[row as usize][ctr_col as usize] = '#';
                }
            }
        });
    for line in ctr {
        for c in line {
            print!("{}", c);
        }
        print!("\n");
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
