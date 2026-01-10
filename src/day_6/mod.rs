pub mod part_1 {
    fn calculate_grand_total(input: &str) -> u64 {
        let grand_total = 0;

        grand_total
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn calculate_grand_total_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(calculate_grand_total(input), 4277556);
        }
    }
}
