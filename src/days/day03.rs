use regex::Regex;
use std::fmt::Display;

use crate::days::Day;

pub struct Day03;

impl Day for Day03 {
    fn part_one(input: &str) -> impl Display {
        let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

        let mut sum: i128 = 0;
        for (_, [arg1, arg2]) in re.captures_iter(input).map(|c| c.extract()) {
            sum += arg1.parse::<i128>().unwrap() * arg2.parse::<i128>().unwrap();
        }

        sum
    }

    fn part_two(input: &str) -> impl Display {
        let re = Regex::new(r"(mul|don't|do)\((?:(\d*),(\d*))?\)").unwrap();

        let mut sum: i128 = 0;
        let mut multiplying = true;

        for captures in re.captures_iter(input) {
            match &captures[1] {
                "mul" => {
                    if multiplying {
                        sum += captures[2].parse::<i128>().unwrap()
                            * captures[3].parse::<i128>().unwrap();
                    }
                }
                "do" => multiplying = true,
                "don't" => multiplying = false,
                _ => (),
            }
        }

        sum
    }

    fn get_day_num() -> u8 {
        return 3;
    }
}
