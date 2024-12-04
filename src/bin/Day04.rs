use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

fn main() -> Result<()> {
    start_day(DAY);

    fn read_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
        reader.lines().map(|l| {
            l.unwrap().chars().collect()
        }).collect()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let puzzle: Vec<Vec<char>> = read_input(reader);

        let mut answer = 0;

        for (y, l) in puzzle.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if c == 'X' {
                    answer += is_xmas(x, y, &puzzle);
                }
            }

        }

        Ok(answer)
    }

    fn is_xmas(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> usize {
        let is_check_right = if check_right(x, y, puzzle) { 1 } else { 0 };
        let is_check_left = if check_left(x, y, puzzle) { 1 } else { 0 };
        let is_check_down = if check_down(x, y, puzzle) { 1 } else { 0 };
        let is_check_up = if check_up(x, y, puzzle) { 1 } else { 0 };
        let is_check_right_down = if check_right_down(x, y, puzzle) { 1 } else { 0 };
        let is_check_left_up = if check_left_up(x, y, puzzle) { 1 } else { 0 };
        let is_check_right_up = if check_right_up(x, y, puzzle) { 1 } else { 0 };
        let is_check_left_down = if check_left_down(x, y, puzzle) { 1 } else { 0 };
        is_check_right + is_check_left + is_check_down + is_check_up + is_check_right_down + is_check_left_up + is_check_right_up + is_check_left_down
    }

    fn check_right(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        puzzle[y].len() > x+3 && puzzle[y][x+1] == 'M' && puzzle[y][x+2] == 'A' && puzzle[y][x+3] == 'S'
    }

    fn check_left(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        x >= 3 && puzzle[y][x-1] == 'M' && puzzle[y][x-2] == 'A' && puzzle[y][x-3] == 'S'
    }

    fn check_down(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        puzzle.len() > y+3 && puzzle[y+1][x] == 'M' && puzzle[y+2][x] == 'A' && puzzle[y+3][x] == 'S'
    }

    fn check_up(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        y >= 3 && puzzle[y-1][x] == 'M' && puzzle[y-2][x] == 'A' && puzzle[y-3][x] == 'S'
    }

    fn check_right_down(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        puzzle[y].len() > x+3 && puzzle.len() > y+3 && puzzle[y+1][x+1] == 'M' && puzzle[y+2][x+2] == 'A' && puzzle[y+3][x+3] == 'S'
    }

    fn check_left_up(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        x >= 3 && y >= 3 && puzzle[y-1][x-1] == 'M' && puzzle[y-2][x-2] == 'A' && puzzle[y-3][x-3] == 'S'
    }

    fn check_right_up(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        puzzle[y].len() > x+3 && y >= 3 && puzzle[y-1][x+1] == 'M' && puzzle[y-2][x+2] == 'A' && puzzle[y-3][x+3] == 'S'
    }

    fn check_left_down(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        x >= 3 && puzzle.len() > y+3 && puzzle[y+1][x-1] == 'M' && puzzle[y+2][x-2] == 'A' && puzzle[y+3][x-3] == 'S'
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let puzzle: Vec<Vec<char>> = read_input(reader);

        let mut answer = 0;

        for (y, l) in puzzle.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if  check_valid_x_mas(c, x, y, &puzzle) {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    fn check_valid_x_mas(c : char, x: usize, y: usize, puzzle: &Vec<Vec<char>>) -> bool {
        c == 'A' && check_valid_mas(x, y, puzzle) && cross_mas_up(x, y, puzzle) && cross_sam_up(x, y, puzzle)
    }

    fn check_valid_mas(x : usize, y: usize, puzzle: &Vec<Vec<char>>) -> bool {
        puzzle[y].len() > x+1 && puzzle.len() > y+1 && x >= 1 && y >= 1
    }

    fn cross_mas_up(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        (puzzle[y-1][x-1] == 'M' && puzzle[y+1][x+1] == 'S') || (puzzle[y-1][x-1] == 'S' && puzzle[y+1][x+1] == 'M')
    }

    fn cross_sam_up(x: usize, y:usize, puzzle: &Vec<Vec<char>>) -> bool {
        (puzzle[y+1][x-1] == 'M' && puzzle[y-1][x+1] == 'S') || (puzzle[y+1][x-1] == 'S' && puzzle[y-1][x+1] == 'M')
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p1 = part1(input_file)?;
    println!("Result Part 1 = {}", result_p1);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result_p2 = part2(input_file)?;
    println!("Result Part 2 = {}", result_p2);

    Ok(())
}