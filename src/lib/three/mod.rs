pub fn execute(lines: Vec<String>) {
    part_one(lines.clone());
    part_two(lines);
}

fn get_bit_values(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let half: u32 = (lines.len() as f32 / 2.0).ceil() as u32;
    let counts = lines
        .iter()
        .fold(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |acc, curr| {
            acc.iter()
                .zip(curr.chars())
                .map(|(&b, v)| b + v.to_digit(10).unwrap())
                .collect()
        });

    let most_common_bit = counts
        .iter()
        .map(|count| {
            if count >= &half {
                "1".to_string()
            } else {
                "0".to_string()
            }
        })
        .collect::<Vec<String>>();

    let least_common_bit = counts
        .iter()
        .map(|count| {
            if count >= &half {
                "0".to_string()
            } else {
                "1".to_string()
            }
        })
        .collect::<Vec<String>>();

    return (most_common_bit, least_common_bit);
}

fn part_one(lines: Vec<String>) {
    let (a, b) = get_bit_values(lines);
    let gamma = isize::from_str_radix(&a.join(""), 2).unwrap();
    let epsilon = isize::from_str_radix(&b.join(""), 2).unwrap();

    println!("part one: {:?}", gamma * epsilon)
}

fn part_two(lines: Vec<String>) {
    let mut oxy_gen_rating = lines.clone();
    let mut iteration = 0;

    while oxy_gen_rating.len() > 1 {
        let (most_sig_count, _) = get_bit_values(oxy_gen_rating.clone());
        oxy_gen_rating = oxy_gen_rating
            .clone()
            .into_iter()
            .filter(|reading| {
                reading.chars().map(String::from).nth(iteration).unwrap()
                    == most_sig_count[iteration]
            })
            .collect();
        iteration += 1;
    }

    let mut scrubber_rating = lines.clone();
    let mut iteration = 0;

    while scrubber_rating.len() > 1 {
        let (_, least_sig_count) = get_bit_values(scrubber_rating.clone());
        scrubber_rating = scrubber_rating
            .clone()
            .into_iter()
            .filter(|reading| {
                reading.chars().map(String::from).nth(iteration).unwrap()
                    == least_sig_count[iteration]
            })
            .collect();
        iteration += 1;
    }

    println!(
        "part two: {:?}",
        isize::from_str_radix(&scrubber_rating[0], 2).unwrap()
            * isize::from_str_radix(&oxy_gen_rating[0], 2).unwrap()
    );
}
