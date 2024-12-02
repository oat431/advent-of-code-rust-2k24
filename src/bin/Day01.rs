use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";

const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");
fn main() -> Result<()> {
    start_day(DAY);
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>();
        // input.iter().for_each(|line| println!("{}", line));

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

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;

    println!("Result = {}", result);

    Ok(())
}