use anyhow::Result;

fn part01() -> Result<String> {
    let input = std::fs::read_to_string("./data/day05.input")?;
    let (drawing, moves) = input.split_once("\n\n").unwrap();
    let number_of_piles = drawing.chars().last().unwrap().to_digit(10).unwrap() as usize;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_piles];
    let last_line = drawing.lines().into_iter().last().unwrap();
    let number_of_lines = drawing.lines().count();
    for s in 1..number_of_piles+1 {
        let pos = last_line.find(&s.clone().to_string()).unwrap();
        for i in (0..number_of_lines-1).rev() {
            let container = drawing.lines().into_iter().nth(i).unwrap().chars().nth(pos);
            match container {
                Some(element) if element != ' ' => stacks[s-1].push(element),
                Some(_) => (),
                None => (),
            }
        }
    }
    moves
        .trim()
        .split("\n")
        .map(|line| line.split_ascii_whitespace().filter_map(|word| word.parse().ok()).collect::<Vec<usize>>())
        .for_each(|line| for _ in 0..line[0] {
            let element = stacks[(line[1]-1) as usize].pop().unwrap();
            stacks[(line[2]-1) as usize].push(element);
        });
    Ok(stacks
       .into_iter()
       .map(|mut stack| stack.pop().unwrap())
       .collect::<String>())
}

fn part02() -> Result<String> {
    let input = std::fs::read_to_string("./data/day05.input")?;
    let (drawing, moves) = input.split_once("\n\n").unwrap();
    let number_of_piles = drawing.chars().last().unwrap().to_digit(10).unwrap() as usize;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_piles];
    let last_line = drawing.lines().into_iter().last().unwrap();
    let number_of_lines = drawing.lines().count();
    for s in 1..number_of_piles+1 {
        let pos = last_line.find(&s.clone().to_string()).unwrap();
        for i in (0..number_of_lines-1).rev() {
            let container = drawing.lines().into_iter().nth(i).unwrap().chars().nth(pos);
            match container {
                Some(element) if element != ' ' => stacks[s-1].push(element),
                Some(_) => (),
                None => (),
            }
        }
    }
    moves
        .trim()
        .split("\n")
        .map(|line| line.split_ascii_whitespace().filter_map(|word| word.parse().ok()).collect::<Vec<usize>>())
        .for_each(|line| {
            let split_index = stacks[(line[1]-1) as usize].len() - line[0];
            let elements = stacks[(line[1]-1) as usize].split_off(split_index);
            stacks[(line[2]-1) as usize].extend(elements);
        });
    Ok(stacks
       .into_iter()
       .map(|mut stack| stack.pop().unwrap())
       .collect::<String>())
}

fn main() -> Result<()> {
    println!("{}", part01().unwrap());
    println!("{}", part02().unwrap());
    Ok(())
}
