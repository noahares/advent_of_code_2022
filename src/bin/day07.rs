use anyhow::Result;

fn part01() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day07.input")?.to_string();
    let lines: Vec<&str> = input.split("\n")
        .collect();
    let mut sums = vec![0; 1];
    let mut result = 0;
    let mut depth = 0;
    for line in lines {
        match line {
            _ if line == "$ cd .." => {
                if sums[depth] < 100000 {
                    result += sums[depth];
                }
                sums[depth - 1] += sums[depth];
                depth -= 1;
                sums.pop();
            },
            _ if line.starts_with("$ cd") => {
                sums.push(0);
                depth += 1;
            },
            _ if sums[depth] > 100000 => (),
            _ if line.starts_with("dir") => (),
            _ if line.starts_with("$ ls") => (),
            file => sums[depth] = sums[depth] + file.split_whitespace().next().unwrap().parse::<u32>().unwrap(),
        }
    }
    Ok(result)
}

fn part02() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day07.input")?.to_string();
    let lines: Vec<&str> = input.split("\n")
        .collect();
    let mut sums = vec![0; 1];
    let mut dir_sizes = Vec::new();
    let mut depth = 0;
    for &line in lines[1..lines.len()-1].into_iter() {
        match line.to_string() {
            _ if line == "$ cd .." => {
                sums[depth - 1] += sums[depth];
                dir_sizes.push(sums[depth]);
                depth -= 1;
                sums.pop();
            },
            _ if line.starts_with("$ cd") => {
                sums.push(0);
                depth += 1;
            },
            _ if line.starts_with("$ ls") => (),
            _ if line.starts_with("dir") => (),
            file => sums[depth] = sums[depth] + file.split_whitespace().next().unwrap().parse::<u32>().unwrap(),
        }
    }
    // ugly directory size reconstruction
    dir_sizes.extend(&sums[1..sums.len()]);
    let total_size: u32 = sums.into_iter().sum();
    dir_sizes.push(total_size);
    dir_sizes.sort();
    for sum in dir_sizes {
        if total_size - sum < 40000000 {
            return Ok(sum)
        }
    }
    Ok(0)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
