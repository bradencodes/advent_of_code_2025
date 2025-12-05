use std::{fs::File, io::BufReader};

pub fn get_file_reader(file_path: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
