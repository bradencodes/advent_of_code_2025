use std::ops::RangeInclusive;

pub fn solve(input: &str) -> u64 {
    let id_ranges = parse_id_ranges(input);

    let mut total_of_invalid_ids = 0;
    for id_range in id_ranges {
        for id in id_range {
            let is_id_valid = get_is_id_valid(id);
            if !is_id_valid {
                total_of_invalid_ids += id
            };
        }
    }

    total_of_invalid_ids
}

fn get_is_id_valid(id: u64) -> bool {
    let id = id.to_string();
    if id.len() % 2 != 0 {
        return true;
    };
    let (first_half, last_half) = id.split_at(id.len() / 2);
    if first_half == last_half {
        return false;
    };

    true
}

fn parse_id_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(",")
        .map(|range_string| {
            let (start, end) = range_string.split_once("-").unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();

            start..=end
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_gives_correct_solution() {
        let test_input = include_str!("test_input.txt");
        let solution = solve(test_input);
        assert_eq!(solution, 1227775554);
    }
}
