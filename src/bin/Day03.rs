use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use anyhow::*;
use regex::Regex;
use adv_code_2024::start_day;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

fn main() -> Result<()> {
    start_day(DAY);
    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let input = reader.lines().flatten();
        let mut line = String::new();
        for l in input {
            line.push_str(&l);
        }

        let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
        let mut sum: i64 = 0;

        for cap in regex.captures_iter(&line) {
            let mul = cap.get(0).unwrap().as_str();
            sum += do_mul_instruction(mul);
        }

        Ok(sum)
    }

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let input = reader.lines().flatten();
        let mut line = String::new();
        for l in input {
            line.push_str(&l);
        }

        let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do(n't)?\(\)")?;
        let mut sum: i64 = 0;
        let mut is_enabled = true;

        for cap in regex.captures_iter(&line) {
            let mul = cap.get(0).unwrap().as_str();
            if mul == "don't()" {
                is_enabled = false;
            } else if mul == "do()" {
                is_enabled = true;
            }
            if is_enabled {
                sum += do_mul_instruction(mul);
            }
        }

        Ok(sum)
    }

    fn do_mul_instruction(mul: &str) -> i64 {
        if mul == "do()" || mul == "don't()" {
            return 0;
        }
        let res = mul.trim_start_matches("mul(").trim_end_matches(")");
        let nums: Vec<&str> = res.split(',').collect();
        let num1 = nums[0].parse::<i64>().unwrap();
        let num2 = nums[1].parse::<i64>().unwrap();
        num1 * num2
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p1 = part1(input_file)?;

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p2 = part2(input_file)?;

    println!("Part 1: {}", result_p1);
    println!("Part 2: {}", result_p2);


    Ok(())
}