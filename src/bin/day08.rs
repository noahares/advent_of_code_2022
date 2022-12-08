use std::collections::HashSet;

use anyhow::Result;

fn part01() -> Result<u32> {
    let matrix: Vec<Vec<u32>> = std::fs::read_to_string("./data/day08.input")?
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let dim = matrix.len();
    for i in 1..dim-1 {
        let mut prefix_max = matrix[i][0];
        for j in 1..dim-1 {
            let item = matrix[i][j];
            if item > prefix_max {
                visible_trees.insert((i, j));
                prefix_max = item;
            }
        }
        prefix_max = matrix[i][dim-1];
        for j in (1..dim-1).rev() {
            let item = matrix[i][j];
            if item > prefix_max {
                visible_trees.insert((i, j));
                prefix_max = item;
            }
        }
    }

    for i in 1..dim-1 {
        let mut prefix_max = matrix[0][i];
        for j in 1..dim-1 {
            let item = matrix[j][i];
            if item > prefix_max {
                visible_trees.insert((j, i));
                prefix_max = item;
            }
        }
        prefix_max = matrix[dim-1][i];
        for j in (1..dim-1).rev() {
            let item = matrix[j][i];
            if item > prefix_max {
                visible_trees.insert((j, i));
                prefix_max = item;
            }
        }
    }
    Ok((visible_trees.len() + 4 * (dim-1)) as u32)
}

fn part02() -> Result<u32> {
    let matrix: Vec<Vec<u32>> = std::fs::read_to_string("./data/day08.input")?
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let dim = matrix.len();
    let mut scenic_scores: Vec<Vec<u32>> = vec![vec![0; dim]; dim];
    for i in 1..dim-1 {
        for j in 1..dim-1 {
            let item = matrix[i][j];
            let mut score = 1;
            let (mut k, mut l) = (i-1, j-1);
            while k > 0  {
                if matrix[k][j] >= item {
                    break;
                }
                k -= 1;
            }
            score *= i - k;

            k = i+1;
            while k < dim-1  {
                if matrix[k][j] >= item {
                    break;
                }
                k += 1;
            }
            score *= k - i;

            while l > 0  {
                if matrix[i][l] >= item {
                    break;
                }
                l -= 1;
            }
            score *= j - l;

            l = j+1;
            while l < dim-1  {
                if matrix[i][l] >= item {
                    break;
                }
                l += 1;
            }
            score *= l - j;

            scenic_scores[i][j] = score as u32;
        }
    }

    Ok(*scenic_scores.iter().map(|row| row.iter().max().unwrap()).max().unwrap() as u32)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
