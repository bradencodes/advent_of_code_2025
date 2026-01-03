use std::ops::RangeInclusive;

fn count_fresh_ingredients(input: &str) -> u32 {
    let fresh_ingredient_count = 0;

    let (fresh_ranges, ingredients) = {
        let split_input: Vec<&str> = input.split("\n\n").collect();
        let fresh_ranges: Vec<RangeInclusive<u32>> = {
            split_input[0]
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|range| {
                    let split_range: Vec<&str> = range.split('-').collect();
                    let range_start = split_range[0].parse().unwrap();
                    let range_end = split_range[1].parse().unwrap();
                    range_start..=range_end
                })
                .collect()
        };
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
}
