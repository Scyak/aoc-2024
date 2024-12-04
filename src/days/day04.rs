use std::{fmt::Display, vec};

use crate::days::Day;

pub struct Day04;

impl Day for Day04 {
    fn part_one(input: &str) -> impl Display {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut xmas_sum = 0;
        for (y, line) in grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'X' {
                    for direction in get_search_directions(x, y, grid.len(), line.len()) {
                        if xmas_in_direction(x as isize, y as isize, direction, &grid) {
                            xmas_sum += 1;
                        }
                    }
                }
            }
        }

        xmas_sum
    }

    fn part_two(input: &str) -> impl Display {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut masx_sum = 0;
        for (y, y_window) in grid.windows(3).enumerate() {
            for (x, _) in y_window[1].windows(3).enumerate() {
                if grid[y + 1][x + 1] == 'A' {
                    if check_masx(x + 1, y + 1, &grid) {
                        masx_sum += 1;
                    }
                }
            }
        }

        masx_sum
    }

    fn get_day_num() -> u8 {
        return 4;
    }
}

fn get_search_directions(x: usize, y: usize, ylen: usize, xlen: usize) -> Vec<(isize, isize)> {
    let mut ydirections = vec![0];
    let mut xdirections = vec![0];

    if y >= 3 {
        ydirections.push(-1);
    }

    if x >= 3 {
        xdirections.push(-1);
    }

    if y <= (ylen - 4) {
        ydirections.push(1);
    }

    if x <= (xlen - 4) {
        xdirections.push(1);
    }

    let mut directions = vec![];

    for y in &ydirections {
        for x in &xdirections {
            directions.push((*x, *y));
        }
    }

    directions.retain(|dir| *dir != (0, 0));

    directions
}

fn xmas_in_direction(x: isize, y: isize, direction: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
    if grid[(y + direction.1) as usize][(x + direction.0) as usize] == 'M'
        && grid[(y + (direction.1 * 2)) as usize][(x + (direction.0 * 2)) as usize] == 'A'
        && grid[(y + (direction.1 * 3)) as usize][(x + (direction.0 * 3)) as usize] == 'S'
    {
        return true;
    }

    false
}

fn check_masx(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    let mas_upper_left = grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S';
    let sam_upper_left = grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M';
    let mas_lower_left = grid[y + 1][x - 1] == 'M' && grid[y - 1][x + 1] == 'S';
    let sam_lower_left = grid[y + 1][x - 1] == 'S' && grid[y - 1][x + 1] == 'M';

    (mas_upper_left || sam_upper_left) && (mas_lower_left || sam_lower_left)
}
