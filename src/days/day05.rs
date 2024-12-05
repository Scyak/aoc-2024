use multimap::MultiMap;
use std::{collections::HashSet, fmt::Display};

use crate::days::Day;

pub struct Day05;

impl Day for Day05 {
    fn part_one(input: &str) -> impl Display {
        let (rules, page_lists) = parse_input(input);

        page_lists
            .into_iter()
            .filter(|pages| first_page_in_wrong_order(pages, &rules) == None)
            .map(|pages| pages[pages.len() / 2])
            .sum::<u32>()
    }

    fn part_two(input: &str) -> impl Display {
        let (rules, page_lists) = parse_input(input);

        let incorrect: Vec<Vec<u32>> = page_lists
            .into_iter()
            .filter(|pages| first_page_in_wrong_order(pages, &rules) != None)
            .collect();

        let mut middle_page_sum = 0;
        for mut pages in incorrect {
            while let Some((page, prereq)) = first_page_in_wrong_order(&pages, &rules) {
                let page_idx = pages.iter().position(|x| *x == page).unwrap();
                let prereq_idx = pages.iter().position(|x| *x == prereq).unwrap();
                pages.swap(page_idx, prereq_idx);
            }

            middle_page_sum += pages[pages.len() / 2];
        }

        middle_page_sum
    }

    fn get_day_num() -> u8 {
        return 5;
    }
}

/// returns a MultiMap of rules (page to prerequisites) and a Vec<Vec<u32>> for the page lists
fn parse_input(input: &str) -> (MultiMap<u32, u32>, Vec<Vec<u32>>) {
    let (rules_list, page_str) = input
        .split_once("\n\n")
        .expect("Couldn't find separation between rules and page lists!");

    let mut rules = MultiMap::new();

    for rule in rules_list.lines() {
        let pages: Vec<u32> = rule
            .split('|')
            .map(|pagenum| pagenum.parse().unwrap())
            .collect();
        rules.insert(pages[1], pages[0]);
    }

    let page_lists = page_str
        .lines()
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    (rules, page_lists)
}

/// returns None if all pages are in the correct order,
/// or Some(page, prerequisite) where page is the page that had the issue and prerequisite is the page that should have been before it
fn first_page_in_wrong_order(pages: &Vec<u32>, rules: &MultiMap<u32, u32>) -> Option<(u32, u32)> {
    let mut printed = HashSet::new();
    for page in pages.iter() {
        match rules.get_vec(&page) {
            None => (),
            Some(prerequisites) => {
                for prereq in prerequisites {
                    if pages.contains(prereq) && !printed.contains(prereq) {
                        return Some((*page, *prereq));
                    }
                }
            }
        }
        printed.insert(*page);
    }
    None
}
