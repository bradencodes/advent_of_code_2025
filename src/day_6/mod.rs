pub mod part_1 {
    enum Operator {
        Add,
        Multiply,
    }

    struct Problem {
        numbers: Vec<u64>,
        operator: Operator,
    }

    fn calculate_grand_total(input: &str) -> u64 {
        let results = {
            let problem_grid: Vec<Vec<&str>> = input
                .split("\n")
                .filter(|string| !string.is_empty())
                .map(|line| line.split_whitespace().collect())
                .collect();

            // TODO: Iterate through columns of problem_grid backwards from the end to
            // get operator and perform operation on numbers in problem.
        };

        let grand_total = results.iter().sum();

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
