use std::collections::HashSet;

pub fn part1() {
    let output = include_str!("./real_input.txt")
        .lines()
        .fold(0, |acc, line| {
            let (winning_numbers, given_numbers) = {
                let mut winning_and_given = line
                    .split(": ")
                    .nth(1)
                    .expect("Missing half after card numbers")
                    .split(" | ")
                    .map(|str| {
                        str.split_whitespace()
                            .map(|item| item.parse::<i32>().expect("Could not parse as int"))
                    });
                (
                    winning_and_given
                        .next()
                        .expect("Missing winning numbers")
                        .collect::<HashSet<_>>(),
                    winning_and_given.next().expect("Missing given numbers"),
                )
            };
            let line_points = given_numbers.fold(0, |acc, num| {
                if winning_numbers.contains(&num) {
                    if acc == 0 {
                        return 1;
                    }
                    return acc * 2;
                }
                return acc;
            });
            acc + line_points
        });

    dbg!(output);
}

pub fn part2() {}
