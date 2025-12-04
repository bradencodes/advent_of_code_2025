use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DIAL_SIZE: i32 = 100;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_1_input.txt")?;
    // let file = File::open("./test_input.txt")?;
    // let file = File::open("./test_input_2.txt")?;
    let reader = BufReader::new(file);

    let mut current_position = 50;
    let mut total_zeros = 0;

    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
            continue;
        };

        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.parse().unwrap();

        let new_magnitude = match direction {
            "R" => current_position + distance,
            "L" => current_position - distance,
            _ => continue,
        };

        total_zeros += {
            if new_magnitude == 0 {
                1
            } else {
                (new_magnitude as f64 / DIAL_SIZE as f64).floor().abs() as i32
            }
        };

        // remove duplicate count in case where dial is at 0 and is turned left
        if current_position == 0 && new_magnitude < 0 {
            total_zeros -= 1
        };

        current_position = new_magnitude.rem_euclid(DIAL_SIZE);
    }

    println!("{total_zeros}");

    Ok(())
}
