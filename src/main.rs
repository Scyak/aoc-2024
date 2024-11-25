mod days;

use anyhow::{bail, Result};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand};
use days::*;

const YEAR: i32 = 2024;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run your solution (specify the day or leave blank to run today,)
    Run {
        #[arg(
            value_parser = clap::value_parser!(u32).range(1..=25),
            help = "The day you want to run (or today by default)")
        ]
        day: Option<u32>,
    },
    /// Download the input from adventofcode.org (specify the day or leave blank to fetch today)
    Fetch {
        #[arg(
            value_parser = clap::value_parser!(u32).range(1..=25),
            help = "The the day you want to fetch the input for (or today by default)")
        ]
        day: Option<u32>,
    },
}

fn today() -> Result<u32> {
    match Local::now() {
        date if date.year() == YEAR
            && date.month() == 12
            && date.day() >= 1
            && date.day() <= 25 =>
        {
            Ok(date.day())
        }
        date => bail!(
            "{}-{}-{} is not a valid AoC {} day!",
            date.year(),
            date.month(),
            date.day(),
            YEAR
        ),
    }
}

fn run_day(day: &Option<u32>) {
    let day = match day {
        Some(day) => *day,
        None => match today() {
            Ok(day) => day,
            Err(e) => panic!("Couldn't run today: {}", e),
        },
    };
    match day {
        1 => day01::Day01::run(),
        2 => day02::Day02::run(),
        3 => day03::Day03::run(),
        4 => day04::Day04::run(),
        5 => day05::Day05::run(),
        6 => day06::Day06::run(),
        7 => day07::Day07::run(),
        8 => day08::Day08::run(),
        9 => day09::Day09::run(),
        10 => day10::Day10::run(),
        11 => day11::Day11::run(),
        12 => day12::Day12::run(),
        13 => day13::Day13::run(),
        14 => day14::Day14::run(),
        15 => day15::Day15::run(),
        16 => day16::Day16::run(),
        17 => day17::Day17::run(),
        18 => day18::Day18::run(),
        19 => day19::Day19::run(),
        20 => day20::Day20::run(),
        21 => day21::Day21::run(),
        22 => day22::Day22::run(),
        23 => day23::Day23::run(),
        24 => day24::Day24::run(),
        25 => day25::Day25::run(),
        day => panic!("The Advent of Code doesn't have a day {day}"),
    }
}

fn fetch_day(_day: &Option<u32>) {
    todo!("No fetching yet")
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { day }) => run_day(day),
        Some(Commands::Fetch { day }) => fetch_day(day),
        None => run_day(&None),
    }
}
