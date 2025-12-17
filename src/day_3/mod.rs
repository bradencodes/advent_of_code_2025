fn find_largest_joltage_part_1(bank: &str) -> u32 {
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

        // If the digit is larger than or equal to the largest_first_num, set largest_second_num to MAX(largest_first_num, largest_second_num), and set largest_first_num to the current digit
        if digit >= largest_first_num {
            largest_second_num = largest_first_num.max(largest_second_num);
            largest_first_num = digit;
        }
    }

    // return the number made from the largest_first_num and largest_second_num
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

        total_joltage += find_largest_joltage_part_1(bank);
    }

    total_joltage
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn find_largest_joltage_works() {
        assert_eq!(find_largest_joltage_part_1("987654321111111"), 98);
        assert_eq!(find_largest_joltage_part_1("811111111111119"), 89);
        assert_eq!(find_largest_joltage_part_1("234234234234278"), 78);
        assert_eq!(find_largest_joltage_part_1("818181911112111"), 92);
        assert_eq!(
            find_largest_joltage_part_1(
                "2232133233333122223222321121432322323324333234233221423334362333113343833132233313523312322224432234"
            ),
            85
        );
        assert_eq!(find_largest_joltage_part_1("911829"), 99);
        assert_eq!(find_largest_joltage_part_1("4111151111"), 51);
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

fn find_largest_joltage_part_2(bank: &str, size: usize) -> String {
    if size == 0 {
        return "".to_owned();
    };

    // For the "n" digit result, find the leftmost largest digit at least "n - 1" from the end.
    // Remove the bank up to and including the index of that digit, and repeat for "n - 1".
    let mut largest_digit = 0;
    let mut largest_digit_index = 0;
    for (i, digit) in bank
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .rev()
        .enumerate()
    {
        if i < size - 1 {
            continue;
        }

        if digit >= largest_digit {
            largest_digit = digit;
            largest_digit_index = bank.len() - i - 1;
        }
    }

    let available_bank = bank.split_at(largest_digit_index + 1).1;

    let mut largest_joltage = largest_digit.to_string();
    largest_joltage.push_str(find_largest_joltage_part_2(available_bank, size - 1).as_str());
    largest_joltage
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut total_joltage = 0;
    let banks = input.split("\n");
    for bank in banks {
        if bank.len() < 1 {
            continue;
        }

        total_joltage += find_largest_joltage_part_2(bank, 12)
            .parse::<u64>()
            .unwrap();
    }

    total_joltage
}

#[cfg(test)]
mod part_2_tests {
    use super::*;

    #[test]
    fn find_largest_joltage_part_2_works() {
        assert_eq!(
            find_largest_joltage_part_2("987654321111111", 12),
            "987654321111"
        );
        assert_eq!(
            find_largest_joltage_part_2("811111111111119", 12),
            "811111111119"
        );
        assert_eq!(
            find_largest_joltage_part_2("234234234234278", 12),
            "434234234278"
        );
        assert_eq!(
            find_largest_joltage_part_2("818181911112111", 12),
            "888911112111"
        );
    }

    #[test]
    fn solve_part_2_test_works() {
        let input = include_str!("test_input.txt");
        assert_eq!(solve_part_2(input), 3121910778619);
    }
}
