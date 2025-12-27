type RowIndex = usize;
type ColumnIndex = usize;
type GridIndex = (RowIndex, ColumnIndex);

const ROLL: char = '@';
const ACCESS_LIMIT: u8 = 4;

fn check_if_roll_is_accessible(diagrid: &[Vec<char>], grid_index: GridIndex) -> bool {
    let column_length = diagrid.len();
    let row_length = diagrid[0].len();

    let is_top_edge = grid_index.0 == 0;
    let is_bottom_edge = grid_index.0 == column_length - 1;
    let is_left_edge = grid_index.1 == 0;
    let is_right_edge = grid_index.1 == row_length - 1;

    let mut adjacent_roll_count: u8 = 0;

    let is_roll_top_left =
        !is_top_edge && !is_left_edge && diagrid[grid_index.0 - 1][grid_index.1 - 1] == ROLL;
    if is_roll_top_left {
        adjacent_roll_count += 1
    };

    let is_roll_top = !is_top_edge && diagrid[grid_index.0 - 1][grid_index.1] == ROLL;
    if is_roll_top {
        adjacent_roll_count += 1
    };

    let is_roll_top_right =
        !is_top_edge && !is_right_edge && diagrid[grid_index.0 - 1][grid_index.1 + 1] == ROLL;
    if is_roll_top_right {
        adjacent_roll_count += 1
    };

    let is_roll_left = !is_left_edge && diagrid[grid_index.0][grid_index.1 - 1] == ROLL;
    if is_roll_left {
        adjacent_roll_count += 1
    };

    let is_roll_right = !is_right_edge && diagrid[grid_index.0][grid_index.1 + 1] == ROLL;
    if is_roll_right {
        adjacent_roll_count += 1
    };

    let is_roll_bottom_left =
        !is_bottom_edge && !is_left_edge && diagrid[grid_index.0 + 1][grid_index.1 - 1] == ROLL;
    if is_roll_bottom_left {
        adjacent_roll_count += 1
    };

    let is_roll_bottom = !is_bottom_edge && diagrid[grid_index.0 + 1][grid_index.1] == ROLL;
    if is_roll_bottom {
        adjacent_roll_count += 1
    };

    let is_roll_bottom_right =
        !is_bottom_edge && !is_right_edge && diagrid[grid_index.0 + 1][grid_index.1 + 1] == ROLL;
    if is_roll_bottom_right {
        adjacent_roll_count += 1
    };

    adjacent_roll_count < ACCESS_LIMIT
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn check_if_roll_is_accessible_works() {
        let diagram_string = "....\n.@@@\n.@@@\n.@@@\n....";
        let diagrid: Vec<Vec<char>> = diagram_string
            .split('\n')
            .map(|row| row.chars().collect())
            .collect();

        // ....\n
        // .@@@\n
        // .@@@\n
        // .@@@\n
        // ....\n

        assert_eq!(check_if_roll_is_accessible(&diagrid, (1, 1)), true);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (1, 2)), false);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (1, 3)), true);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (2, 1)), false);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (2, 2)), false);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (2, 3)), false);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (3, 1)), true);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (3, 2)), false);
        assert_eq!(check_if_roll_is_accessible(&diagrid, (3, 3)), true);
    }
}
