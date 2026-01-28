mod part_1 {
    fn find_sizes_of_3_largest_circuits_from_n_connections(input: &str, n_connections: u64) -> u64 {
        let sizes = 1;

        // 1. Parse the input to get all of the junction box locations.
        
        
        // 2. Make an ordered list of all the closest pairs of junction boxes.
        
        
        // 3. Iterate through the closest pairs list the given number of times.
        
        
        //     a. If the pair is not already in the same circuit, connect them.
        
        
        // 4. Find the 3 largest circuits and multiply them. Return this value
        
        
        return sizes;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_sizes_of_3_largest_circuits_from_n_connections_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(
                find_sizes_of_3_largest_circuits_from_n_connections(input, 10),
                40
            );
        }
    }
}
