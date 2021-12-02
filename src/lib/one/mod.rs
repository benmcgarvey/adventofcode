pub fn execute(lines: Vec<String>) {
    let mut lines = lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    part_one(lines.as_mut());
    part_two(lines.as_mut());
}

fn part_one(lines: &mut Vec<i64>) {
    let increase_count = lines
        .windows(2)
        .fold(0, |acc, curr| if curr[1] > curr[0] { acc + 1 } else { acc });

    println!("part one: {:?}", increase_count);
}

fn part_two(lines: &mut Vec<i64>) {
    let mut increase_count = 0;
    lines.windows(3).reduce(|prev, next| {
        if next.iter().sum::<i64>() > prev.iter().sum() {
            increase_count += 1;
        }
        next
    });

    println!("part two: {:?}", increase_count);
}
