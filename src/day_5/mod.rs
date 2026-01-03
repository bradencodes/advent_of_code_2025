use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
struct FreshRange {
    start: u32,
    end: u32,
}

/// Combines and sorts an array of fresh ranges. When fresh ranges overlap, they are
/// combined into a single range that encompasses all of the fresh values.
fn combine_ranges(ranges: &Vec<FreshRange>) -> Vec<FreshRange> {
    let mut combined_ranges: Vec<FreshRange> = Vec::new();

    // 1. Iterate through each range. Find a range from `combined_ranges` that is the
    // closest range that ends before the current range starts. Find a range from
    // `combined_ranges` that is the closest range that starts after the current
    // range ends.
    // 2. All of the ranges between but not including these found ranges overlap with
    // the current range. Extend the current range to line up with the extreme start
    // and end of the overlapping ranges.
    // 3. Insert the current range into the appropriate spot by replacing the first
    // overlapping range. Then remove the rest of the overlapping ranges.

    ranges.to_vec()
}

fn count_fresh_ingredients(input: &str) -> u32 {
    let fresh_ingredient_count = 0;

    let (fresh_ranges, ingredients) = {
        let split_input: Vec<&str> = input.split("\n\n").collect();
        let separate_fresh_ranges: Vec<FreshRange> = {
            split_input[0]
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|range| {
                    let split_range: Vec<&str> = range.split('-').collect();
                    let start = split_range[0].parse().unwrap();
                    let end = split_range[1].parse().unwrap();
                    FreshRange { start, end }
                })
                .collect()
        };

        let fresh_ranges = combine_ranges(&separate_fresh_ranges);

        let ingredients: Vec<u32> = split_input[1]
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|id| id.parse().unwrap())
            .collect();

        (fresh_ranges, ingredients)
    };

    dbg!();

    fresh_ingredient_count
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn count_fresh_ingredients_works() {
        let input = include_str!("./test_input.txt");
        assert_eq!(count_fresh_ingredients(input), 3);
    }

    mod combine_ranges {
        use super::*;

        #[test]
        fn combines_overlapping_ranges() {
            let overlapping_ranges = vec![
                FreshRange { start: 10, end: 14 },
                FreshRange { start: 5, end: 11 },
            ];
            let expected_combined_ranges = vec![FreshRange { start: 5, end: 14 }];
            assert_eq!(
                combine_ranges(&overlapping_ranges),
                expected_combined_ranges
            );
        }

        #[test]
        fn doesnt_combine_separate_ranges() {
            let separate_ranges = vec![
                FreshRange { start: 5, end: 7 },
                FreshRange { start: 10, end: 11 },
            ];
            assert_eq!(combine_ranges(&separate_ranges), separate_ranges);
        }

        #[test]
        fn combines_multiple_overlapping_ranges() {
            let overlapping_ranges = vec![
                FreshRange { start: 1, end: 3 },
                FreshRange { start: 5, end: 7 },
                FreshRange { start: 8, end: 10 },
                FreshRange { start: 12, end: 15 },
                FreshRange { start: 2, end: 9 },
            ];
            let expected_combined_ranges = vec![
                FreshRange { start: 1, end: 10 },
                FreshRange { start: 12, end: 15 },
            ];
            assert_eq!(
                combine_ranges(&overlapping_ranges),
                expected_combined_ranges
            );
        }

        #[test]
        fn sorts_ranges() {
            let overlapping_ranges = vec![
                FreshRange { start: 5, end: 7 },
                FreshRange { start: 12, end: 15 },
                FreshRange { start: 8, end: 10 },
                FreshRange { start: 1, end: 3 },
            ];
            let expected_combined_ranges = vec![
                FreshRange { start: 1, end: 3 },
                FreshRange { start: 5, end: 7 },
                FreshRange { start: 8, end: 10 },
                FreshRange { start: 12, end: 15 },
            ];
            assert_eq!(
                combine_ranges(&overlapping_ranges),
                expected_combined_ranges
            );
        }
    }
}
