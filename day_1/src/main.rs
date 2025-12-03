use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> std::io::Result<()> {
    let file = File::open("../day_1_input.txt")?;
    let reader = BufReader::new(file);
    
    let mut current_position = 50;
    let mut total_zeros = 0;
    
    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 { continue };
        
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.parse().unwrap();
        match direction {
            "R" => current_position = (current_position + distance) % 100,
            "L" => current_position = (current_position - distance) % 100,
            _ => {}
        }
        
        if current_position == 0 {total_zeros += 1};
    }
    
    println!("{total_zeros}");
    
    Ok(())
}