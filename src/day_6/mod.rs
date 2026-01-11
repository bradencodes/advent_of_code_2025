pub mod part_1 {
    enum Operator {
        Add,
        Multiply,
    }

    fn calculate_grand_total(input: &str) -> u64 {
        let mut grand_total = 0;

        let problem_grid: Vec<Vec<&str>> = input
            .split("\n")
            .filter(|string| !string.is_empty())
            .map(|line| line.split_whitespace().collect())
            .collect();

        // TODO: Iterate through columns of problem_grid backwards from the end to
        // get operator and perform operation on numbers in problem.
        let column_length = problem_grid.len();
        let row_length = problem_grid[0].len();

        for row_index in 0..row_length {
            let mut operator: Option<Operator> = None;
            let mut total = 0;
            for column_index in (0..column_length).rev() {
                let problem_part = problem_grid[column_index][row_index];
                if column_index == column_length - 1 {
                    match problem_part {
                        "+" => {
                            operator = Some(Operator::Add);
                            total = 0;
                        }
                        "*" => {
                            operator = Some(Operator::Multiply);
                            total = 1;
                        }
                        _ => panic!("'{problem_part}' is not a valid operator"),
                    }
                } else {
                    let number: u64 = problem_part.parse().expect("part should be a number");
                    match operator {
                        Some(Operator::Add) => total += number,
                        Some(Operator::Multiply) => total *= number,
                        None => unreachable!(),
                    }
                }
            }

            grand_total += total;
        }

        grand_total
    }

    pub fn solve(input: &str) -> u64 {
        calculate_grand_total(input)
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
