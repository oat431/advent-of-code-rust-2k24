use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
use anyhow::*;
use const_format::concatcp;
use adv_code_2024::start_day;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn new() -> Self {
        Self::Up
    }

    const fn invert(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_in_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn is_out_of_bounds(&self, x_len: usize, y_len: usize) -> bool {
        self.x < 0 || self.y < 0 || self.x >= x_len as i32 || self.y >= y_len as i32
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    let result_p1 = execute_with_input(INPUT_FILE, part1)?;
    println!("Part 1 Result = {result_p1}");

    let result_p2 = execute_with_input(INPUT_FILE, part2)?;
    println!("Part 2 Result = {result_p2}");

    Ok(())
}

fn execute_with_input<F>(file_path: &str, func: F) -> Result<usize>
where
    F: FnOnce(BufReader<File>) -> Result<usize>,
{
    let input_file = BufReader::new(File::open(file_path)?);
    func(input_file)
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let input = parse_input(reader);
    Ok(color_path(input).unwrap())
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut answer = 0;

    let input = parse_input(reader);
    let x_len = input[0].len();
    let y_len = input.len();

    for x in 0..x_len {
        for y in 0..y_len {
            let mut test_input = input.clone();
            if input[y][x] != '^' {
                test_input[y][x] = '#';
                if color_path(test_input).is_none() {
                    answer += 1;
                }
            }
        }
    }

    Ok(answer)
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| line.chars().collect())
        .collect()
}

fn initialize_guard_position(input: &mut Vec<Vec<char>>, pos: &mut Position) {
    for (y, row) in input.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if *cell == '^' {
                *cell = 'X';
                *pos = Position::new(x as i32, y as i32);
            }
        }
    }
}

fn color_path(mut input: Vec<Vec<char>>) -> Option<usize> {
    let mut count = 1;
    let x_len = input[0].len();
    let y_len = input.len();

    let mut pos = Position::new(0, 0);
    let mut dir = Direction::new();
    let mut visited = HashSet::new();

    initialize_guard_position(&mut input, &mut pos);

    loop {
        // Stuck in a loop
        if visited.contains(&(pos.x, pos.y, dir)) {
            return None;
        }

        visited.insert((pos.x, pos.y, dir));
        pos.move_in_direction(dir);

        if pos.is_out_of_bounds(x_len, y_len) {
            break;
        }

        match input[pos.y as usize][pos.x as usize] {
            '#' => {
                dir = dir.invert();
                pos.move_in_direction(dir);
                dir = dir.turn_right();
            }
            c if c != 'X' => {
                input[pos.y as usize][pos.x as usize] = 'X';
                count += 1;
            }
            _ => (),
        }
    }

    Some(count)
}
