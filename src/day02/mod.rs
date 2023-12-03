struct CubeCounts {
    red: i32, green: i32, blue: i32,
}

const TOTALS: CubeCounts = CubeCounts { blue: 14, green: 13, red: 12 };

pub fn part1() {
    let input_str = include_str!("./real_input.txt");

    let output = input_str
        .split("\n").filter_map(| line: &str | {
            let (game_str, sets_str) = {
                let mut line_split = line.split(": ");
                (line_split.next().unwrap(), line_split.next().unwrap())
            };
            
            let game_id = game_str.split(" ").last().unwrap().parse::<i32>().unwrap();
            let is_game_valid = sets_str.split("; ").all(|set_str| {
                let mut cubes = CubeCounts { blue: 0, red: 0, green: 0};
                for cube_count_str in set_str.split(", ") {
                    let (amount, color) = {
                        let mut split = cube_count_str.split(" ");
                        (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap())
                    };
                    match color {
                        "red" => cubes.red += amount,
                        "green" => cubes.green += amount,
                        "blue" => cubes.blue += amount,
                        _ => panic!(),
                    };
                }
                let is_set_valid =
                    cubes.blue <= TOTALS.blue &&
                    cubes.green <= TOTALS.green &&
                    cubes.red <= TOTALS.red;
                return is_set_valid;
            });

            if is_game_valid {Some(game_id)} else {None}
        }).sum::<i32>();

    dbg!(output);
}