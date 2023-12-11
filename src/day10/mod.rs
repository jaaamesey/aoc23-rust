use grid::Grid;

const INPUT: &str = include_str!("./test_input.txt");

pub fn part1() {
    let grid = INPUT
        .lines()
        .fold(Grid::<char>::new(0, 0), |mut grid, line| {
            let row = line.chars().collect::<Vec<_>>();
            grid.push_row(row);
            grid
        });

    let starting_pos = (|| {
        for (pos, &char) in grid.indexed_iter() {
            if char == 'S' {
                return (pos.0 as i32, pos.1 as i32);
            }
        }
        panic!();
    })();

    fn get_direction(char: char, last_dir: (i8, i8)) -> Option<(i8, i8)> {
        match char {
            '-' if last_dir.1 == -1 => Some((0, -1)),
            '-' if last_dir.1 == 1 => Some((0, 1)),

            '|' if last_dir.0 == -1 => Some((-1, 0)),
            '|' if last_dir.0 == 1 => Some((1, 0)),

            'F' if last_dir.1 == -1 => Some((1, 0)),
            'F' if last_dir.0 == -1 => Some((0, 1)),

            '7' if last_dir.1 == 1 => Some((1, 0)),
            '7' if last_dir.0 == -1 => Some((0, -1)),

            'L' if last_dir.1 == -1 => Some((-1, 0)),
            'L' if last_dir.0 == 1 => Some((0, 1)),

            'J' if last_dir.1 == 1 => Some((-1, 0)),
            'J' if last_dir.0 == 1 => Some((0, -1)),
            _ => None,
        }
    }

    let mut cursor = (starting_pos.0, starting_pos.1 + 1);
    let mut last_dir = (0, 1); // HARDCODED
    let mut step_count = 1;

    loop {
        let char = grid[(cursor.0 as usize, cursor.1 as usize)];
        let dir_to_move = get_direction(char, last_dir).unwrap();

        dbg!(dir_to_move);

        cursor = (
            cursor.0 + (dir_to_move.0 as i32),
            cursor.1 + (dir_to_move.1 as i32),
        );

        dbg!(cursor);

        last_dir = dir_to_move;

        step_count += 1;

        if cursor == starting_pos {
            break;
        }
    }

    dbg!(step_count / 2);
}

pub fn part2() {}
