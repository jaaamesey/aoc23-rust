const INPUT: &str = include_str!("./real_input.txt");

pub fn part1() {
    let mut lines = INPUT.lines();
    let times = lines
        .next()
        .unwrap()
        .split("Time:")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let record_distances = lines
        .next()
        .unwrap()
        .split("Distance:")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let num_races: usize = times.len();

    let output = (0..num_races).fold(1, |acc, i| {
        let time_limit = times[i];
        let record_distance = record_distances[i];

        let num_record_breaking_release_times = (0..time_limit)
            .filter(|&button_release_time| {
                let mut elapsed_time: i64 = 1;
                let mut velocity: i64 = 0;
                let mut distance_travelled: i64 = 0;

                while elapsed_time <= time_limit {
                    if elapsed_time < button_release_time {
                        velocity += 1;
                    } else {
                        distance_travelled += velocity;
                    }
                    elapsed_time += 1;
                }

                return distance_travelled > record_distance;
            })
            .count();

        return acc * num_record_breaking_release_times;
    });

    dbg!(output);
}

pub fn part2() {
    let mut lines = INPUT.lines();
    let time_limit = lines
        .next()
        .unwrap()
        .split("Time:")
        .last()
        .unwrap()
        .split_whitespace()
        .fold("".to_string(), |acc, curr| acc + curr)
        .parse::<f64>()
        .unwrap();
    let record_distance = lines
        .next()
        .unwrap()
        .split("Distance:")
        .last()
        .unwrap()
        .split_whitespace()
        .fold("".to_string(), |acc, curr| acc + curr)
        .parse::<f64>()
        .unwrap();

    let sqrt_b_squared_minus_4ac = (time_limit * time_limit - (-4.0 * -record_distance)).sqrt();

    // "a" is just -1, which makes making a value negative equivalent to dividing by 2a.
    let negative_intercept = -(-time_limit - sqrt_b_squared_minus_4ac);
    let positive_intercept = -(-time_limit + sqrt_b_squared_minus_4ac);

    let possible_release_points = (negative_intercept - positive_intercept) as i64 / 2;

    dbg!(possible_release_points);
}
