use std::io::{BufRead, BufReader};
use adv_code_2024::start_day;
use const_format::concatcp;
use anyhow::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");
fn main() -> Result<()> {
    start_day(DAY);

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>();
        let mut safe_reports = 0;

        for line in input {
            let range: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            if is_safe_report(&range) {
                safe_reports += 1;
            }
        }
        Ok(safe_reports)
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>();
        let mut safe_reports = 0;

        for line in input {
            let range: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            if is_safe_report(&range) || tolerate_single_level(&range) {
                safe_reports += 1;
            }
        }
        Ok(safe_reports)
    }

    fn tolerate_single_level(nums: &[i32]) -> bool {
        for i in 0..nums.len() {
            let temp: Vec<i32> = nums
                .iter()
                .enumerate()
                .filter_map(|(j, &x)| if i == j { None } else { Some(x)})
                .collect();
            if is_safe_report(&temp) {
                return true;
            }
        }
        false
    }

    fn is_safe_report(nums: &[i32]) -> bool {
        is_increasing(nums) || is_decreasing(nums)
    }

    fn is_increasing(nums: &[i32]) -> bool {
        nums.windows(2).all(|pair| is_fit_in_lesser_than_condition(pair[0], pair[1]))
    }

    fn is_decreasing(nums: &[i32]) -> bool {
        nums.windows(2).all(|pair| is_fit_in_greater_than_condition(pair[0], pair[1]))
    }

    fn is_fit_in_lesser_than_condition(a: i32, b:i32) -> bool {
        let diff = b - a;
        matches!(diff, 1 | 2 | 3)
    }

    fn is_fit_in_greater_than_condition(a: i32, b:i32) -> bool {
        let diff = a - b;
        matches!(diff, 1 | 2 | 3)
    }

    let input_file = BufReader::new(std::fs::File::open(INPUT_FILE)?);
    let result_p1 = part1(input_file)?;

    let input_file = BufReader::new(std::fs::File::open(INPUT_FILE)?);
    let result_p2 = part2(input_file)?;

    println!("=== Part 1 ===");
    println!("Result: {}", result_p1);

    println!("=== Part 1 ===");
    println!("Result: {}", result_p2);


    Ok(())
}