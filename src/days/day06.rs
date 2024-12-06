use std::{fmt::Display, vec};

use pathfinding::matrix::Matrix;

use crate::days::Day;

pub struct Day06;

const OBSTACLE: (isize, isize) = (0, 0);

impl Day for Day06 {
    fn part_one(input: &str) -> impl Display {
        let (grid, guard_position) = parse_input(input);

        visited_locations(&grid, &guard_position).len()
    }

    fn part_two(input: &str) -> impl Display {
        let (grid, guard_position) = parse_input(input);

        visited_locations(&grid, &guard_position)
            .into_iter()
            .filter(|loc| {
                grid.get(*loc).unwrap().is_empty() && check_loop(&grid, &guard_position, *loc)
            })
            .count()
    }

    fn get_day_num() -> u8 {
        return 6;
    }
}

fn visited_locations(
    grid: &Matrix<Vec<(isize, isize)>>,
    guard_position: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut guard_position = *guard_position;
    let mut guard_direction = (-1, 0);
    let mut grid = grid.clone();

    let mut visited = vec![];

    while let Some(next_position) = grid.in_direction(guard_position, guard_direction).next() {
        let pos = grid
            .get_mut(next_position)
            .expect("Position should be in grid but isn't!");

        if !pos.contains(&OBSTACLE) {
            if !visited.contains(&next_position) {
                visited.push(next_position);
            }
            guard_position = next_position;
        } else {
            guard_direction = turn_right(guard_direction);
        }
    }

    visited
}

fn parse_input(input: &str) -> (Matrix<Vec<(isize, isize)>>, (usize, usize)) {
    let grid = Matrix::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|c| match c {
                '^' => vec![(-1, 0)],
                '#' => vec![OBSTACLE],
                _ => vec![],
            })
            .collect::<Vec<Vec<(isize, isize)>>>()
    }))
    .unwrap();

    for (y, line) in grid.iter().enumerate() {
        for (x, pos) in line.iter().enumerate() {
            if pos.contains(&(-1, 0)) {
                return (grid, (y, x));
            }
        }
    }

    panic!("Guard not found!")
}

fn turn_right(guard_direction: (isize, isize)) -> (isize, isize) {
    match guard_direction {
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => panic!("Unexpected guard direction!"),
    }
}

fn check_loop(
    grid: &Matrix<Vec<(isize, isize)>>,
    guard_position: &(usize, usize),
    new_obstacle: (usize, usize),
) -> bool {
    let mut guard_position = *guard_position;
    let mut guard_direction = (-1, 0);

    let mut grid = grid.clone();
    *grid.get_mut(new_obstacle).unwrap() = vec![OBSTACLE];

    while let Some(next_position) = grid.in_direction(guard_position, guard_direction).next() {
        match grid
            .get_mut(next_position)
            .expect("Position should be in grid but isn't!")
        {
            pos if !pos.contains(&guard_direction) && !pos.contains(&OBSTACLE) => {
                pos.push(guard_direction);
                guard_position = next_position;
            }
            pos if pos.contains(&OBSTACLE) => guard_direction = turn_right(guard_direction),
            pos if pos.contains(&guard_direction) => return true,
            _ => panic!("Unexpected element on grid!"),
        }
    }

    false
}
