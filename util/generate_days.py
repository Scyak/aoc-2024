day_base = """use std::fmt::Display;

use crate::days::Day;

pub struct Day{day:02};

impl Day for Day{day:02} {{
    fn part_one(_input: &str) -> impl Display {{
        "no solution yet :("
    }}
    fn part_two(_input: &str) -> impl Display {{
        "no solution yet :("
    }}
    fn get_day_num() -> u8 {{
        return {day};
    }}
}}"""

dayfile = "../src/days/day{day:02}.rs"

for day in range(1, 26):
    current_day = day_base.format(day=day)
    file = dayfile.format(day=day)
    f = open(file, "w")
    f.write(current_day)
    f.close()