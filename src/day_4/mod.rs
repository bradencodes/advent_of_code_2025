type RowIndex = usize;
type ColumnIndex = usize;
type GridIndex = (RowIndex, ColumnIndex);

mod diagrid {
    pub type Diagrid = Vec<Vec<char>>;
    pub const ROLL: char = '@';
    pub const EMPTY: char = '.';
}

const ACCESS_LIMIT: u8 = 4;

mod count_grid {
    pub type AdjacentRollsCount = i32;
    pub type CountGrid = Vec<Vec<AdjacentRollsCount>>;
    pub const EMPTY: AdjacentRollsCount = -1;
}

use count_grid::{AdjacentRollsCount, CountGrid};
use diagrid::Diagrid;

fn check_if_roll_is_accessible(diagrid: &Diagrid, grid_index: GridIndex) -> bool {
    count_adjacent_rolls(diagrid, grid_index) < ACCESS_LIMIT
}

fn create_diagrid(input: &str) -> Diagrid {
    input
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().collect())
        .collect()
}

fn count_accessible_rolls(input: &str) -> u32 {
    let diagrid = create_diagrid(input);
    let mut accessible_row_count = 0;

    for row_index in 0..diagrid.len() {
        for column_index in 0..diagrid[0].len() {
            if diagrid[row_index][column_index] == diagrid::ROLL {
                let is_roll_accessible =
                    check_if_roll_is_accessible(&diagrid, (row_index, column_index));

                if is_roll_accessible {
                    accessible_row_count += 1
                };
            }
        }
    }

    accessible_row_count
}

pub fn solve_part_1(input: &str) -> u32 {
    count_accessible_rolls(input)
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn check_if_roll_is_accessible_works() {
        let input = "....\n.@@@\n.@@@\n.@@@\n....\n";
        let diagrid = create_diagrid(input);

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

    #[test]
    fn count_accessible_rolls_works() {
        let input = include_str!("./test_input.txt");
        assert_eq!(count_accessible_rolls(input), 13);
    }
}

fn count_adjacent_rolls(diagrid: &Diagrid, grid_index: GridIndex) -> u8 {
    let column_length = diagrid.len();
    let row_length = diagrid[0].len();

    let is_top_edge = grid_index.0 == 0;
    let is_bottom_edge = grid_index.0 == column_length - 1;
    let is_left_edge = grid_index.1 == 0;
    let is_right_edge = grid_index.1 == row_length - 1;

    let mut adjacent_roll_count: u8 = 0;

    let is_roll_top_left = !is_top_edge
        && !is_left_edge
        && diagrid[grid_index.0 - 1][grid_index.1 - 1] == diagrid::ROLL;
    if is_roll_top_left {
        adjacent_roll_count += 1
    };

    let is_roll_top = !is_top_edge && diagrid[grid_index.0 - 1][grid_index.1] == diagrid::ROLL;
    if is_roll_top {
        adjacent_roll_count += 1
    };

    let is_roll_top_right = !is_top_edge
        && !is_right_edge
        && diagrid[grid_index.0 - 1][grid_index.1 + 1] == diagrid::ROLL;
    if is_roll_top_right {
        adjacent_roll_count += 1
    };

    let is_roll_left = !is_left_edge && diagrid[grid_index.0][grid_index.1 - 1] == diagrid::ROLL;
    if is_roll_left {
        adjacent_roll_count += 1
    };

    let is_roll_right = !is_right_edge && diagrid[grid_index.0][grid_index.1 + 1] == diagrid::ROLL;
    if is_roll_right {
        adjacent_roll_count += 1
    };

    let is_roll_bottom_left = !is_bottom_edge
        && !is_left_edge
        && diagrid[grid_index.0 + 1][grid_index.1 - 1] == diagrid::ROLL;
    if is_roll_bottom_left {
        adjacent_roll_count += 1
    };

    let is_roll_bottom =
        !is_bottom_edge && diagrid[grid_index.0 + 1][grid_index.1] == diagrid::ROLL;
    if is_roll_bottom {
        adjacent_roll_count += 1
    };

    let is_roll_bottom_right = !is_bottom_edge
        && !is_right_edge
        && diagrid[grid_index.0 + 1][grid_index.1 + 1] == diagrid::ROLL;
    if is_roll_bottom_right {
        adjacent_roll_count += 1
    };

    adjacent_roll_count
}

fn for_adjacent_spaces<F>(count_grid: &mut CountGrid, grid_index: GridIndex, mut callback: F)
where
    F: FnMut(&mut CountGrid, GridIndex),
{
    let column_length = count_grid.len();
    let row_length = count_grid[0].len();

    let is_top_edge = grid_index.0 == 0;
    let is_bottom_edge = grid_index.0 == column_length - 1;
    let is_left_edge = grid_index.1 == 0;
    let is_right_edge = grid_index.1 == row_length - 1;

    let is_space_top_left = !is_top_edge && !is_left_edge;
    if is_space_top_left {
        let top_left_grid_index = (grid_index.0 - 1, grid_index.1 - 1);
        callback(count_grid, top_left_grid_index)
    };

    let is_space_top = !is_top_edge;
    if is_space_top {
        let top_grid_index = (grid_index.0 - 1, grid_index.1);
        callback(count_grid, top_grid_index)
    };

    let is_space_top_right = !is_top_edge && !is_right_edge;
    if is_space_top_right {
        let top_right_grid_index = (grid_index.0 - 1, grid_index.1 + 1);
        callback(count_grid, top_right_grid_index)
    };

    let is_space_left = !is_left_edge;
    if is_space_left {
        let left_grid_index = (grid_index.0, grid_index.1 - 1);
        callback(count_grid, left_grid_index)
    };

    let is_space_right = !is_right_edge;
    if is_space_right {
        let right_grid_index = (grid_index.0, grid_index.1 + 1);
        callback(count_grid, right_grid_index)
    };

    let is_space_bottom_left = !is_bottom_edge && !is_left_edge;
    if is_space_bottom_left {
        let bottom_left_grid_index = (grid_index.0 + 1, grid_index.1 - 1);
        callback(count_grid, bottom_left_grid_index)
    };

    let is_space_bottom = !is_bottom_edge;
    if is_space_bottom {
        let bottom_grid_index = (grid_index.0 + 1, grid_index.1);
        callback(count_grid, bottom_grid_index)
    };

    let is_space_bottom_right = !is_bottom_edge && !is_right_edge;
    if is_space_bottom_right {
        let bottom_right_grid_index = (grid_index.0 + 1, grid_index.1 + 1);
        callback(count_grid, bottom_right_grid_index)
    };
}

fn remove_roll(count_grid: &mut CountGrid, grid_index: GridIndex) -> u32 {
    let mut removed_rolls_count = 0;

    count_grid[grid_index.0][grid_index.1] = count_grid::EMPTY;
    removed_rolls_count += 1;

    for_adjacent_spaces(count_grid, grid_index, |count_grid, adjacent_grid_index| {
        let space = &mut count_grid[adjacent_grid_index.0][adjacent_grid_index.1];
        if *space != count_grid::EMPTY {
            let adjacent_rolls_count = space;
            *adjacent_rolls_count -= 1;
            let is_roll_removeable = get_is_roll_removeable(adjacent_rolls_count);
            if is_roll_removeable {
                let more_removed_rolls_count = remove_roll(count_grid, adjacent_grid_index);
                removed_rolls_count += more_removed_rolls_count;
            }
        }
    });

    removed_rolls_count
}

fn get_is_roll_removeable(adjacent_rolls_count: &AdjacentRollsCount) -> bool {
    (0..=3).contains(adjacent_rolls_count)
}

fn count_removeable_rolls(input: &str) -> u32 {
    let mut removeable_rolls_count = 0;

    // 1. Create new count_grid by iterating through diagrid. For every "." put a -1.
    // For every "@" put the number of adjacent rolls.
    // 2. Iterate through count_grid. For every number equal to or between 0 and 3, set
    // it to -1 and subtract 1 from each adjacent non-negative space. Then subtract
    // 1 from the number of final rolls. Repeat for each adjacent non-negative space.

    let diagrid = create_diagrid(input);
    let mut count_grid: CountGrid = diagrid
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(column_index, char)| match *char {
                    diagrid::EMPTY => count_grid::EMPTY,
                    diagrid::ROLL => {
                        count_adjacent_rolls(&diagrid, (row_index, column_index)) as i32
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let grid_row_length = count_grid.len();
    let grid_column_length = count_grid[0].len();

    for row_index in 0..grid_row_length {
        for column_index in 0..grid_column_length {
            let adjacent_rolls_count: &AdjacentRollsCount = &count_grid[row_index][column_index];
            let is_roll_removeable = get_is_roll_removeable(adjacent_rolls_count);
            if is_roll_removeable {
                let removed_rolls_count = remove_roll(&mut count_grid, (row_index, column_index));
                removeable_rolls_count += removed_rolls_count;
            }
        }
    }

    removeable_rolls_count
}

pub fn solve_part_2(input: &str) -> u32 {
    count_removeable_rolls(input)
}

#[cfg(test)]
mod part_2_tests {
    use super::*;

    #[test]
    fn count_removeable_rolls_works() {
        let input = include_str!("./test_input.txt");
        assert_eq!(count_removeable_rolls(input), 43);
    }
}
