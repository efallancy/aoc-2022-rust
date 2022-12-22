use std::fs;

pub fn read_input_day(day: u32) -> String {
    return fs::read_to_string(format!("./input/day{day}.txt")).expect("File to exist");
}
