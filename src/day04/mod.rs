pub fn part1() {
    let output = include_str!("./test_input.txt")
        .lines()
        .fold(0, |acc, line| {
            let (winning_numbers, given_numbers) = {
                let mut winning_and_given = line
                    .split(": ")
                    .nth(1)
                    .expect("Missing half after card numbers")
                    .split(" | ")
                    .map(|str| str.split_whitespace());
                (
                    winning_and_given.next().expect("Missing winning numbers"),
                    winning_and_given.next().expect("Missing given numbers"),
                )
            };
            let line_points = given_numbers.fold(0, |acc, num| {
                if winning_numbers.clone().any(|n| n == num) {
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

pub fn part2() {
    let mut total_scratchcards = 0;

    let lines = include_str!("./real_input.txt").lines();

    let mut num_cards_left = vec![1; lines.clone().count()];

    lines.enumerate().for_each(|(i, line)| {
        let (winning_numbers, given_numbers) = {
            let mut winning_and_given = line
                .split(": ")
                .nth(1)
                .expect("Missing half after card numbers")
                .split(" | ")
                .map(|str| str.split_whitespace());
            (
                winning_and_given.next().expect("Missing winning numbers"),
                winning_and_given.next().expect("Missing given numbers"),
            )
        };
        let num_matches = given_numbers
            .filter(|num| winning_numbers.clone().any(|n| n == *num))
            .count();

        while num_cards_left[i] > 0 {
            total_scratchcards += 1;

            let mut cards_to_copy = num_matches;

            num_cards_left[i] -= 1;

            while cards_to_copy > 0 {
                num_cards_left[i + cards_to_copy] += 1;
                cards_to_copy -= 1;
            }
        }
    });
    dbg!(total_scratchcards);
}

pub fn _part1_andrew() {
    let output = include_str!("./real_input.txt")
        .lines()
        .fold(0, |points, line| {
            let [winners, havers] = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" | ")
                .collect::<Vec<_>>()[..]
            else {
                todo!()
            };
            let win_vec: Vec<_> = winners.split_whitespace().collect();
            let have_vec: Vec<_> = havers.split_whitespace().collect();
            let mut line_points = 0;

            for val in have_vec {
                if win_vec.contains(&val) {
                    if line_points == 0 {
                        line_points = 1;
                    } else {
                        line_points *= 2;
                    }
                }
            }
            return points + line_points;
        });
    dbg!(output);
}
