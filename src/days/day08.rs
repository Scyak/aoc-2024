use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use multimap::MultiMap;
use prime_factorization::Factorization;

use crate::days::Day;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    fn step(&self, x_step: isize, y_step: isize) -> Coordinates {
        Coordinates {
            x: self.x + x_step,
            y: self.y + y_step,
        }
    }

    fn within_bounds(&self, xlen: isize, ylen: isize) -> bool {
        self.x >= 0 && self.x < xlen && self.y >= 0 && self.y < ylen
    }
}

pub struct Day08;

impl Day for Day08 {
    fn part_one(input: &str) -> impl Display {
        let antennas = parse_input(input);
        let ylen = input.lines().count() as isize;
        let xlen = input.lines().next().unwrap().len() as isize;

        let mut antinodes = HashSet::new();
        for frequency in antennas.keys() {
            for antenna in antennas
                .get_vec(frequency)
                .unwrap()
                .iter()
                .tuple_combinations::<(&Coordinates, &Coordinates)>()
            {
                if antenna.0 != antenna.1 {
                    let x_distance = antenna.1.x - antenna.0.x;
                    let y_distance = antenna.1.y - antenna.0.y;

                    // first antinode: "before" first node
                    let antinode_before = antenna.0.step(x_distance * -1, y_distance * -1);
                    if antinode_before.within_bounds(xlen, ylen) {
                        antinodes.insert(antinode_before);
                    }

                    // second antinode: "after" second node
                    let antinode_after = antenna.1.step(x_distance, y_distance);
                    if antinode_after.within_bounds(xlen, ylen) {
                        antinodes.insert(antinode_after);
                    }

                    // for general cases, there should also be possible antinodes between the antennas,
                    // but the problem statement makes clear that isn't possible
                }
            }
        }

        antinodes.len()
    }

    fn part_two(input: &str) -> impl Display {
        let antennas = parse_input(input);
        let ylen = input.lines().count() as isize;
        let xlen = input.lines().next().unwrap().len() as isize;

        let mut antinodes = HashSet::new();
        for frequency in antennas.keys() {
            for antenna in antennas
                .get_vec(frequency)
                .unwrap()
                .iter()
                .tuple_combinations::<(&Coordinates, &Coordinates)>()
            {
                if antenna.0 != antenna.1 {
                    let mut x_step = antenna.1.x - antenna.0.x;
                    let mut y_step = antenna.1.y - antenna.0.y;

                    // get smallest integer step
                    // turns out (at least for my input) this does nothing as distances never have common factors or a zero component,
                    // but it makes for a more general solution
                    let y_distance_factors = Factorization::run(y_step.abs() as u32).factors;
                    for factor in Factorization::run(x_step.abs() as u32).factors {
                        if y_distance_factors.contains(&factor) {
                            x_step = x_step / (factor as isize);
                            y_step = y_step / (factor as isize);
                        }
                    }

                    if x_step == 0 {
                        y_step = 1;
                    }

                    if y_step == 0 {
                        x_step = 1;
                    }

                    // move in positive steps from first antenna
                    let mut coords = antenna.0.clone();
                    while coords.within_bounds(xlen, ylen) {
                        let new_coords = coords.step(x_step, y_step);
                        antinodes.insert(coords);
                        coords = new_coords;
                    }

                    // move in negative steps from first antenna
                    let mut coords = antenna.0.clone();
                    while coords.within_bounds(xlen, ylen) {
                        let new_coords = coords.step(x_step * -1, y_step * -1);
                        antinodes.insert(coords);
                        coords = new_coords;
                    }
                }
            }
        }

        antinodes.len()
    }

    fn get_day_num() -> u8 {
        return 8;
    }
}

fn parse_input(input: &str) -> MultiMap<char, Coordinates> {
    let mut map = MultiMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert(
                    c,
                    Coordinates {
                        x: x as isize,
                        y: y as isize,
                    },
                );
            }
        }
    }

    map
}
