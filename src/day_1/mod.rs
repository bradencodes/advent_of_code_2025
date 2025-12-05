pub fn solve() -> i32 {
    let file = include_str!("input.txt");

    const DIAL_SIZE: i32 = 100;

    let mut current_position = 50;
    let mut total_zero_clicks = 0;

    for line in file.lines() {
        let line = line;

        // line length must be at least 2 characters, the direction and distance
        let is_line_length_invalid = line.len() < 2;
        if is_line_length_invalid {
            continue;
        };

        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.parse().unwrap();

        let full_rotations = distance / DIAL_SIZE;
        total_zero_clicks += full_rotations;

        let partial_rotation_distance = distance % DIAL_SIZE;
        let next_position = match direction {
            "R" => current_position + partial_rotation_distance,
            "L" => current_position - partial_rotation_distance,
            _ => continue,
        };
        if current_position != 0 && (next_position < 1 || next_position > 99) {
            total_zero_clicks += 1
        };

        current_position = next_position.rem_euclid(DIAL_SIZE);
    }

    total_zero_clicks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_gives_correct_solution() {
        let solution = solve();
        assert_eq!(solution, 6623);
    }
}
