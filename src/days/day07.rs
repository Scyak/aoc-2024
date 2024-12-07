use std::fmt::Display;

use crate::days::Day;

pub struct Day07;

impl Day for Day07 {
    fn part_one(input: &str) -> impl Display {
        parse_input(input)
            .into_iter()
            .filter(|(result, factors)| check_result(result, factors[0], &factors[1..], false))
            .map(|(result, _)| result)
            .sum::<u64>()
    }

    fn part_two(input: &str) -> impl Display {
        parse_input(input)
            .into_iter()
            .filter(|(result, factors)| check_result(result, factors[0], &factors[1..], true))
            .map(|(result, _)| result)
            .sum::<u64>()
    }

    fn get_day_num() -> u8 {
        return 7;
    }
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (result, factors) = line.split_once(": ").unwrap();
            (
                result.parse().unwrap(),
                factors
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn check_result(result: &u64, previous_step: u64, factors: &[u64], part_two: bool) -> bool {
    if factors.len() == 0 {
        if previous_step == *result {
            return true;
        } else {
            return false;
        }
    }

    let mut mul_result = false;
    if previous_step * factors[0] <= *result {
        mul_result = check_result(result, previous_step * factors[0], &factors[1..], part_two);
    }

    let mut add_result = false;
    if previous_step + factors[0] <= *result {
        add_result = check_result(result, previous_step + factors[0], &factors[1..], part_two);
    }

    let mut concat_result = false;
    if part_two && concat(previous_step, factors[0]) <= *result {
        concat_result = check_result(
            result,
            concat(previous_step, factors[0]),
            &factors[1..],
            part_two,
        );
    }

    mul_result || add_result || concat_result
}

fn concat(front: u64, back: u64) -> u64 {
    front * (10u64.pow(back.ilog(10) + 1)) + back
}
