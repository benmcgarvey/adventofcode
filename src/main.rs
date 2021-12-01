use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let mut lines = lines_from_file("./src/input.txt");

    part_one(lines.as_mut());
    part_two(lines.as_mut());
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|string| string.parse::<i64>().unwrap())
        .collect()
}

fn part_one(lines: &mut Vec<i64>) {
    let increase_count = lines
        .windows(2)
        .fold(0, |acc, curr| if curr[1] > curr[0] { acc + 1 } else { acc });

    println!("part one increase count: {:?}", increase_count);
}

fn part_two(lines: &mut Vec<i64>) {
    let mut increase_count = 0;
    lines.windows(3).reduce(|prev, next| {
        if next.iter().sum::<i64>() > prev.iter().sum() {
            increase_count += 1;
        }
        next
    });

    println!("part two increase count: {:?}", increase_count);
}
