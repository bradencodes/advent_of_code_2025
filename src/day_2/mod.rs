use std::ops::RangeInclusive;

pub fn solve_part_1(input: &str) -> u64 {
    let id_ranges = parse_id_ranges(input);

    fn get_is_id_valid_part_1(id: u64) -> bool {
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

    let mut total_of_invalid_ids = 0;
    for id_range in id_ranges {
        for id in id_range {
            let is_id_valid = get_is_id_valid_part_1(id);
            if !is_id_valid {
                total_of_invalid_ids += id
            };
        }
    }

    total_of_invalid_ids
}

fn get_is_id_valid_part_2(id: u64) -> bool {
    let id_string = id.to_string();
    let id_length = id_string.len();
    let largest_step_size = id_length / 2;
    for step_size in (1..=largest_step_size).rev() {
        // an invalid id must have a sequence that repeats a whole number of times
        if id_length % step_size != 0 {
            continue;
        }

        let sequence_repeats_at_step_size = {
            let num_of_chunks = id_length / step_size;

            (0..step_size).all(|offset| {
                let are_all_nums_same_in_each_chunk_at_offset = {
                    let first_chunk_num = id_string.get(offset..=offset).unwrap();
                    (1..=num_of_chunks).all(|chunk| {
                        let index_to_check = (chunk - 1) * step_size + offset;
                        let current_chunk_num =
                            id_string.get(index_to_check..=index_to_check).unwrap();
                        current_chunk_num == first_chunk_num
                    })
                };

                are_all_nums_same_in_each_chunk_at_offset
            })
        };

        if sequence_repeats_at_step_size {
            return false;
        };
    }

    true
}

pub fn solve_part_2(input: &str) -> u64 {
    let id_ranges = parse_id_ranges(input);

    let mut total_of_invalid_ids = 0;
    for id_range in id_ranges {
        for id in id_range {
            let is_id_valid = get_is_id_valid_part_2(id);
            if !is_id_valid {
                total_of_invalid_ids += id
            }
        }
    }

    total_of_invalid_ids
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
mod part_1_tests {
    use super::*;

    #[test]
    fn solve_part_1_gives_correct_solution() {
        let test_input = include_str!("test_input.txt");
        let solution = solve_part_1(test_input);
        assert_eq!(solution, 1227775554);
    }
}

#[cfg(test)]
mod part_2_tests {
    use super::*;

    #[test]
    fn get_is_id_valid_part_2_works() {
        let invalid_id = 1188511885;
        assert_eq!(get_is_id_valid_part_2(invalid_id), false);
    }

    #[test]
    fn solve_part_2_gives_correct_solution() {
        let test_input = include_str!("test_input.txt");
        let solution = solve_part_2(test_input);
        assert_eq!(solution, 4174379265);
    }
}
