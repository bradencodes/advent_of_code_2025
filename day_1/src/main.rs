use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DIAL_SIZE: i32 = 100;

fn get_any_zero_clicks(reader: BufReader<File>) -> std::io::Result<i32> {
    let mut current_position = 50;
    let mut total_zero_clicks = 0;

    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
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

fn main() -> std::io::Result<()> {
    let file = File::open("./day_1_input.txt")?;
    let reader = BufReader::new(file);

    if let Ok(total_zero_clicks) = get_any_zero_clicks(reader) {
        println!("{total_zero_clicks}");
    }

    Ok(())
}
