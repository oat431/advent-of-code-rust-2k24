extern crate core;

use adv_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

fn main() -> Result<()> {
    start_day(DAY);

    fn prepare_input<R: BufRead>(reader: R) -> Result<Vec<Vec<String>>> {
        let mut rules = vec![];
        let mut pages = vec![];
        let mut reading_pages = false;

        for line in reader.lines() {
            let line = line?;

            if line.trim().is_empty() {
                reading_pages = true;
            } else {
                if reading_pages {
                    pages.push(line);
                } else {
                    rules.push(line);
                }
            }
        }

        let mut input = vec![];
        input.push(rules);
        input.push(pages);

        Ok(input)

    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let input = prepare_input(reader)?;
        let rules = input.get(0).unwrap();
        let pages = input.get(1).unwrap();

        for line in pages {
            let pages = get_pages(line);
            if is_correct_order(&pages, &rules) {
                let mid = pages.get(pages.len() / 2).unwrap();
                answer += mid;
            }
        }

        Ok(answer)
    }



    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let input = prepare_input(reader)?;
        let rules = input.get(0).unwrap();
        let pages = input.get(1).unwrap();

        for line in pages {
            let mut pages = get_pages(line);

            if !is_correct_order(&pages, &rules) {
                pages = swap(pages, &rules);
                let mid = pages.get(pages.len() / 2).unwrap();
                answer += mid;
            }
        }

        Ok(answer)
    }

    fn get_pages(line: &str) -> Vec<usize> {
        line.split(',').map(|x| x.parse().unwrap()).collect()
    }

    fn is_correct_order(pages: &[usize], rules: &[String]) -> bool {
        for (i, page) in pages.iter().enumerate() {
            for j in 0..i {
                let test = format!("{page}|{}", pages.get(j).unwrap());
                if rules.contains(&&test) {
                    return false;
                }
            }
        }
        true
    }

    fn swap(mut pages: Vec<usize>, rules: &[String]) -> Vec<usize> {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..pages.len() - 1 {
                let test = format!("{}|{}", pages[i + 1], pages[i]);
                if rules.contains(&test) {
                    pages.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
        pages
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p1 = part1(input_file)?;
    println!("Result part 1= {result_p1}");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p2 = part2(input_file)?;
    println!("Result part 2 = {result_p2}");

    Ok(())
}

