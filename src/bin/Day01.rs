use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";

const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");
fn main() -> Result<()> {
    start_day(DAY);

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>();

        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        for lines in input.iter() {
            let line = lines;
            let parts: Vec<&str> = line.split_whitespace().collect();
            let l : i32 = parts[0].parse::<i32>()?;
            let r : i32 = parts[1].parse::<i32>()?;
            left.push(l);
            right.push(r);
        }

        left.sort();
        right.sort();

        let distances: i32 = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum();
        Ok(distances as usize)
    }


    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>();
        let mut left: HashSet<i32> = HashSet::new();
        let mut right: Vec<i32> = Vec::new();

        for lines in input.iter() {
            let line = lines;
            let parts: Vec<&str> = line.split_whitespace().collect();
            let l : i32 = parts[0].parse::<i32>()?;
            let r : i32 = parts[1].parse::<i32>()?;
            left.insert(l);
            right.push(r);
        }

        let mut similar_score: HashMap<i32, i32> = HashMap::new();
        for &l in &left {
            similar_score.insert(l, 0);
        }

        for &r in &right {
            if let Some(score) = similar_score.get_mut(&r) {
                *score += 1;
            }
        }

        let sum: i32 = similar_score.iter().map(|(&key, &value)| key * value).sum();

        Ok(sum as usize)
    }



    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p1 = part1(input_file)?;

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p2 = part2(input_file)?;

    println!("=== Part 1 ===");
    println!("Result = {}", result_p1);

    println!("=== Part 2 ===");
    println!("Result = {}", result_p2);

    Ok(())
}