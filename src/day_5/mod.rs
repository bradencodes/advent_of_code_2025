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

    for range in ranges {
        let mut current_range = range.clone();

        if combined_ranges.is_empty() {
            combined_ranges.push(current_range);
            continue;
        }

        // Find the rightmost range where r.end < current_range.start
        // This is the last range that ends before the current range starts
        let before_idx = {
            let mut left = 0;
            let mut right = combined_ranges.len();
            let mut result = None;

            while left < right {
                let mid = left + (right - left) / 2;

                if combined_ranges[mid].end < current_range.start {
                    // This range ends before current starts, so it's a candidate
                    result = Some(mid);
                    // Look for a later range that also satisfies the condition
                    left = mid + 1;
                } else {
                    // This range doesn't end before current starts
                    right = mid;
                }
            }

            result
        };

        // Find the leftmost range where r.start > current_range.end
        // This is the first range that starts after the current range ends
        let after_idx = {
            let mut left = 0;
            let mut right = combined_ranges.len();
            let mut result = None;

            while left < right {
                let mid = left + (right - left) / 2;

                if combined_ranges[mid].start > current_range.end {
                    // This range starts after current ends, so it's a candidate
                    result = Some(mid);
                    // Look for an earlier range that also satisfies the condition
                    right = mid;
                } else {
                    // This range doesn't start after current ends
                    left = mid + 1;
                }
            }

            result
        };

        // Determine the overlapping range indices
        let (first_overlap_idx, last_overlap_idx) = match (before_idx, after_idx) {
            (Some(before), Some(after)) => {
                if before + 1 >= after {
                    // No overlapping ranges
                    (None, None)
                } else {
                    (Some(before + 1), Some(after - 1))
                }
            }
            (Some(before), None) => {
                if before + 1 < combined_ranges.len() {
                    (Some(before + 1), Some(combined_ranges.len() - 1))
                } else {
                    (None, None)
                }
            }
            (None, Some(after)) => {
                if after > 0 {
                    (Some(0), Some(after - 1))
                } else {
                    (None, None)
                }
            }
            (None, None) => (Some(0), Some(combined_ranges.len() - 1)),
        };

        // If there are overlapping ranges, extend the current range
        if let (Some(first), Some(last)) = (first_overlap_idx, last_overlap_idx) {
            let min_start = combined_ranges[first].start.min(current_range.start);

            let max_end = combined_ranges[last].end.max(current_range.end);

            current_range = FreshRange {
                start: min_start,
                end: max_end,
            };

            // Replace the first overlapping range with the extended range
            combined_ranges[first] = current_range;

            // Remove the rest of the overlapping ranges
            if last > first {
                combined_ranges.drain(first + 1..=last);
            }
        } else {
            // No overlapping ranges, insert at the appropriate position
            let insert_pos = before_idx.map(|i| i + 1).unwrap_or(0);
            combined_ranges.insert(insert_pos, current_range);
        }
    }

    combined_ranges
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
