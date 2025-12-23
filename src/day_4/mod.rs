fn check_if_roll_is_accessible(diagrid: [[char]], index: usize) -> bool {
    let row_length = diagrid[0].len()
    let is_top_left_open = {
        let top_left_index = index - row_length - 1;
        let is_top_left_off_edge =
            top_left_index < index || top_left_index % row_length >= index % row_length;
    };

    true
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    // ....\n
    // .@@@\n
    // .@@@\n
    // .@@@\n
    // ....\n

    #[test]
    fn check_if_roll_is_accessible_works() {
        let diagram_string = "....\n.@@@\n.@@@\n.@@@\n....\n";
        let diagrid = diagram_string.split('\n').map(|row| row.chars()).collect();
        assert_eq!(check_if_roll_is_accessible(diagrid, 5), true);
        assert_eq!(check_if_roll_is_accessible(diagrid, 6), false);
        assert_eq!(check_if_roll_is_accessible(diagrid, 7), true);
        assert_eq!(check_if_roll_is_accessible(diagrid, 9), false);
        assert_eq!(check_if_roll_is_accessible(diagrid, 10), false);
        assert_eq!(check_if_roll_is_accessible(diagrid, 11), false);
        assert_eq!(check_if_roll_is_accessible(diagrid, 13), true);
        assert_eq!(check_if_roll_is_accessible(diagrid, 14), false);
        assert_eq!(check_if_roll_is_accessible(diagrid, 15), true);
    }
}
