fn find_largest_joltage_errant(bank: &str) -> u32 {
    // initialize the largest_first_num and largest_second_num to the second to last
    // and last digits in the bank, respectively.
    let mut largest_first_num: u32 = 0;
    let mut largest_second_num: u32 = 0;

    // iterate through each digit in the bank in reverse order. For each iteration...
    for (i, char) in bank.chars().rev().enumerate() {
        let digit: u32 = char.to_digit(10).unwrap();
        if i == 0 {
            largest_second_num = digit;
            continue;
        }
        if i == 1 {
            largest_first_num = digit;
            continue;
        }

        // If the digit is larger than the largest_first_num, set largest_second_num to MAX(largest_first_num, largest_second_num), and set largest_first_num to the current digit
        if digit > largest_first_num {
            largest_second_num = largest_first_num.max(largest_second_num);
            largest_first_num = digit;
        }
    }

    // return the number made from the largest_first_num and largest_second_num
    let largest_joltage = largest_first_num * 10 + largest_second_num;
    largest_joltage
}

fn find_largest_joltage(bank: &str) -> u32 {
    let mut largest_first_num = 0;
    let mut largest_first_num_index = 0;
    for (i, char) in bank.split_at(bank.len() - 1).0.chars().rev().enumerate() {
        let digit = char.to_digit(10).unwrap();
        if digit >= largest_first_num {
            largest_first_num = digit;
            largest_first_num_index = bank.len() - i - 2;
        }
    }

    let mut largest_second_num = 0;
    for char in bank.split_at(largest_first_num_index + 1).1.chars() {
        let digit = char.to_digit(10).unwrap();
        if digit > largest_second_num {
            largest_second_num = digit
        };
    }

    let largest_joltage = largest_first_num * 10 + largest_second_num;
    largest_joltage
}

pub fn solve_part_1(input: &str) -> u32 {
    let mut total_joltage = 0;
    let banks = input.split("\n");
    for bank in banks {
        if bank.len() < 1 {
            continue;
        }

        total_joltage += find_largest_joltage(bank);
    }

    total_joltage
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn find_largest_joltage_works() {
        assert_eq!(find_largest_joltage("987654321111111"), 98);
        assert_eq!(find_largest_joltage("811111111111119"), 89);
        assert_eq!(find_largest_joltage("234234234234278"), 78);
        assert_eq!(find_largest_joltage("818181911112111"), 92);
        assert_eq!(
            find_largest_joltage(
                "2232133233333122223222321121432322323324333234233221423334362333113343833132233313523312322224432234"
            ),
            85
        );
        assert_eq!(find_largest_joltage("911829"), 99);
        assert_eq!(find_largest_joltage("4111151111"), 51);
    }

    #[test]
    fn solve_part_1_test_works() {
        let input = include_str!("test_input.txt");
        assert_eq!(solve_part_1(input), 357);
    }

    #[test]
    fn solve_part_1_works() {
        let input = include_str!("input.txt");
        assert_eq!(solve_part_1(input), 17311);
    }
}
