use std::fmt::Display;

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    fn part_one(input: &str) -> impl Display {
        let mut list_one = vec![];
        let mut list_two = vec![];

        for line in input.lines() {
            let locations: Vec<i64> = line
                .split_whitespace()
                .map(|num: &str| num.parse().unwrap())
                .collect();
            list_one.push(locations[0]);
            list_two.push(locations[1]);
        }

        list_one.sort();
        list_two.sort();

        let mut total_difference = 0;

        for (index, location) in list_one.iter().enumerate() {
            total_difference += (location - list_two[index]).abs();
        }

        total_difference
    }
    fn part_two(input: &str) -> impl Display {
        let mut list_one = vec![];
        let mut list_two = vec![];

        for line in input.lines() {
            let locations: Vec<i64> = line
                .split_whitespace()
                .map(|num: &str| num.parse().unwrap())
                .collect();
            list_one.push(locations[0]);
            list_two.push(locations[1]);
        }

        let mut similiarity = 0;

        for location in list_one.iter() {
            let occurences = list_two.iter().filter(|loc2| *loc2 == location).count();
            similiarity += location * occurences as i64;
        }

        similiarity
    }
    fn get_day_num() -> u8 {
        return 1;
    }
}
