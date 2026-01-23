fn parse_input(input: &str) -> (usize, Vec<Vec<usize>>) {
    let mut parsed = input
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

    let initial_beam_location = parsed.next().unwrap()[0];
    let splitter_locations: Vec<Vec<usize>> = parsed.collect();

    (initial_beam_location, splitter_locations)
}

pub mod part_1 {
    use super::*;
    use std::collections::HashSet;

    fn count_split_beams(input: &str) -> u64 {
        let mut split_beam_count = 0;

        // Create a set to keep track of the indices of beams, and start with the one
        // at the top. Go down each row and if there's a beam splitter at the same
        // index as a beam add 1 to split_beam_count, add the -1 and +1 indices of
        // the current index and remove the current index from the set.
        let (initial_beam_location, splitter_locations) = parse_input(input);

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

pub mod part_2 {
    use super::*;

    fn count_timelines(input: &str) -> u64 {
        // Create a vec the length of a row to keep track of how many beams are at
        // each index. Add 1 to the vec for the starting beam. Go down each row and for each
        // beam splitter, add the timeline count at that index to the adjacent indices and set
        // the current index to 0. Add up all the values at the end to get the timelines count.
        let mut beam_counts: Vec<u64> = {
            let manifold_width = input.lines().next().unwrap().chars().count();
            vec![0; manifold_width]
        };

        let (initial_beam_location, splitter_locations) = parse_input(input);
        beam_counts[initial_beam_location] = 1;

        for line in splitter_locations {
            for splitter in line {
                let beam_count_at_splitter = beam_counts[splitter];
                beam_counts[splitter - 1] += beam_count_at_splitter;
                beam_counts[splitter + 1] += beam_count_at_splitter;
                beam_counts[splitter] = 0;
            }
        }

        let timelines_count = beam_counts.iter().sum();

        timelines_count
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn count_timelines_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(count_timelines(input), 40);
        }
    }
}
