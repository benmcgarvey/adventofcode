pub fn execute(mut lines: Vec<String>) {
    part_one(lines.as_mut());
    part_two(lines.as_mut());
}

fn part_one(lines: &mut Vec<String>) {
    // (horizontal, vertical)
    let position = lines.iter().fold((0, 0), |acc, curr| {
        let mut iter = curr.split_whitespace();
        let direction = iter.next().unwrap();
        let magnitude = iter.next().unwrap().parse::<i64>().unwrap();

        match direction {
            "forward" => (acc.0 + magnitude, acc.1),
            "up" => (acc.0, acc.1 - magnitude),
            "down" => (acc.0, acc.1 + magnitude),
            _ => acc,
        }
    });

    println!("part one: {:?}", position.0 * position.1);
}

fn part_two(lines: &mut Vec<String>) {
    // (horizontal, vertical, aim)
    let position = lines.iter().fold((0, 0, 0), |acc, curr| {
        let mut iter = curr.split_whitespace();
        let direction = iter.next().unwrap();
        let magnitude = iter.next().unwrap().parse::<i64>().unwrap();

        match direction {
            "forward" => (acc.0 + magnitude, acc.1 + (magnitude * acc.2), acc.2),
            "up" => (acc.0, acc.1, acc.2 - magnitude),
            "down" => (acc.0, acc.1, acc.2 + magnitude),
            _ => acc,
        }
    });

    println!("part two: {:?}", position.0 * position.1);
}
