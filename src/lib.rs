use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day_1() -> std::io::Result<i32> {
    let file = File::open("./day_1_input.txt")?;
    let reader = BufReader::new(file);

    const DIAL_SIZE: i32 = 100;

    let mut current_position = 50;
    let mut total_zero_clicks = 0;

    for line in reader.lines() {
        let line = line?;

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

    Ok(total_zero_clicks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_day_1_gives_correct_solution() {
        if let Ok(solution) = solve_day_1() {
            assert_eq!(solution, 6623);
        }
    }
}
