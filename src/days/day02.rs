use std::fmt::Display;

use crate::days::Day;

pub struct Day02;

impl Day for Day02 {
    fn part_one(input: &str) -> impl Display {
        let mut safe_sum = 0;

        for line in input.lines() {
            let report = line
                .split_whitespace()
                .map(|num: &str| num.parse().unwrap())
                .collect();

            if is_safe(&report) {
                safe_sum += 1;
            }
        }

        safe_sum
    }

    fn part_two(input: &str) -> impl Display {
        let mut safe_sum = 0;

        for line in input.lines() {
            let report = line
                .split_whitespace()
                .map(|num: &str| num.parse().unwrap())
                .collect();

            if is_safe_with_dampener(&report) {
                safe_sum += 1;
            }
        }

        safe_sum
    }

    fn get_day_num() -> u8 {
        return 2;
    }
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let descending = report[1] < report[0];

    for window in report.windows(2) {
        if levels_safe(window[0], window[1], descending) {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for index in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(index);
        if is_safe(&new_report) {
            return true;
        }
    }

    false
}

fn levels_safe(previous_level: i32, current_level: i32, descending: bool) -> bool {
    let distance = (current_level - previous_level).abs();
    (current_level < previous_level) != descending || distance > 3 || distance < 1
}
