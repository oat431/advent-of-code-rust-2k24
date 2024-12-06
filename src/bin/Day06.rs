use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::start_day;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
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
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn from_reader<R: BufRead>(reader: R) -> Result<Self> {
        let cells = reader
            .lines()
            .map(|l| l.map_err(Error::from))
            .map(|line| line.map(|l| l.chars().collect()))
            .collect::<Result<Vec<Vec<char>>>>()?;
        Ok(Self { cells })
    }

    fn width(&self) -> usize {
        self.cells.first().map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 || x as usize >= self.width() || y as usize >= self.height() {
            None
        } else {
            Some(self.cells[y as usize][x as usize])
        }
    }

    fn set(&mut self, x: i32, y: i32, c: char) {
        if x >= 0 && y >= 0 && (x as usize) < self.width() && (y as usize) < self.height() {
            self.cells[y as usize][x as usize] = c;
        }
    }

    fn clone_grid(&self) -> Self {
        Self {
            cells: self.cells.clone(),
        }
    }

    fn find_guard_start(&mut self) -> Option<Position> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.cells[y][x] == '^' {
                    self.cells[y][x] = 'X';
                    return Some(Position::new(x as i32, y as i32));
                }
            }
        }
        None
    }
}

fn move_guard(pos: &mut Position, dir: Direction) {
    match dir {
        Direction::Up => pos.y -= 1,
        Direction::Right => pos.x += 1,
        Direction::Down => pos.y += 1,
        Direction::Left => pos.x -= 1,
    };
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn color_path(mut grid: Grid) -> Option<usize> {
    let mut count = 1;
    let mut dir = Direction::new();
    let mut visited = HashSet::new();

    let start = grid.find_guard_start()?;
    let (x_len, y_len) = (grid.width() as i32, grid.height() as i32);

    let mut pos = start;

    loop {
        // stuck in the loop
        if visited.contains(&(pos.x, pos.y, dir)) {
            return None;
        }
        visited.insert((pos.x, pos.y, dir));

        move_guard(&mut pos, dir);

        // If guard leaves the area, we are done
        if pos.x < 0 || pos.y < 0 || pos.x >= x_len || pos.y >= y_len {
            break;
        }

        match grid.get(pos.x, pos.y) {
            Some('#') => {
                // Hit a wall: turn around and then turn right
                dir = dir.invert();
                move_guard(&mut pos, dir);
                dir = dir.turn_right();
            }
            Some(c) if c != 'X' => {
                // Mark visited cell
                grid.set(pos.x, pos.y, 'X');
                count += 1;
            }
            _ => {
                // Already visited cell or something else: just continue
            }
        }
    }

    Some(count)
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let grid = Grid::from_reader(reader)?;
    Ok(color_path(grid).unwrap())
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let original_grid = Grid::from_reader(reader)?;
    let (x_len, y_len) = (original_grid.width(), original_grid.height());

    let mut answer = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            let mut grid = original_grid.clone_grid();
            if grid.cells[y][x] != '^' {
                grid.set(x as i32, y as i32, '#');
                if color_path(grid).is_none() {
                    answer += 1;
                }
            }
        }
    }

    Ok(answer)
}

fn main() -> Result<()> {
    start_day(DAY);

    // Part 1
    println!("=== Part 1 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {result}");

    // Part 2
    println!("=== Part 2 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {result}");

    Ok(())
}
