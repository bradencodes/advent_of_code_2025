pub mod part_1 {
    fn count_split_beams(input: &str) -> u64 {
        let split_beam_count = 0;

        // Create a set to keep track of the indices of beams, and start with the one
        // at the top. Go down each row and if there's a beam splitter at the same
        // index as a beam add 1 to split_beam_count, add the -1 and +1 indices of
        // the current index and remove the current index from the set.

        split_beam_count
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
