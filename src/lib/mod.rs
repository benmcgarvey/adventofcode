use std::{
    fs::File,
    io::{BufRead, BufReader},
};
mod one;
mod three;
mod two;

pub fn run(day: i8) {
    match day {
        1 => one::execute(lines_from_file("one")),
        2 => two::execute(lines_from_file("two")),
        3 => three::execute(lines_from_file("three")),
        _ => println!("Not implemented"),
    }
}

fn lines_from_file(day: &str) -> Vec<String> {
    let file = File::open(format!("./src/lib/{}/input.txt", day)).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
