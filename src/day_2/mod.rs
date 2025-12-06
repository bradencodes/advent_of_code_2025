use std::ops::{Range, RangeInclusive};

pub fn solve() {
    let input = include_str!("input.txt");

    let id_ranges = parse_id_ranges(input);
}

fn parse_id_ranges(input: &str) -> Vec<RangeInclusive<i32>> {
    input
        .split(",")
        .map(|range_string| {
            let (start, end) = range_string.split_once("-").unwrap();
            let start: i32 = start.parse().unwrap();
            let end: i32 = end.parse().unwrap();

            start..=end
        })
        .collect()
}
