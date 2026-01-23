pub mod part_1 {
    use std::collections::HashSet;

    fn count_split_beams(input: &str) -> u64 {
        let mut split_beam_count = 0;

        // Create a set to keep track of the indices of beams, and start with the one
        // at the top. Go down each row and if there's a beam splitter at the same
        // index as a beam add 1 to split_beam_count, add the -1 and +1 indices of
        // the current index and remove the current index from the set.
        let mut splitter_locations = input
            .lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .fold(Vec::new(), |mut acc, (index, char)| match char {
                        '.' => acc,
                        _ => {
                            acc.push(index);
                            acc
                        }
                    })
            })
            .filter(|line| !line.is_empty());

        let initial_beam_location = splitter_locations.next().unwrap()[0];
        let mut beams = HashSet::new();
        beams.insert(initial_beam_location);

        for line in splitter_locations {
            for splitter in line {
                if beams.contains(&splitter) {
                    split_beam_count += 1;
                    beams.remove(&splitter);
                    beams.insert(splitter - 1);
                    beams.insert(splitter + 1);
                }
            }
        }

        split_beam_count
    }

    pub fn solve(input: &str) -> u64 {
        count_split_beams(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn count_split_beams_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(count_split_beams(input), 21);
        }
    }
}
