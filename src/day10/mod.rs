use geo::polygon;
use geo::Area;
use geo_types::{coord, LineString, Polygon};
use grid::Grid;

const INPUT: &str = include_str!("./test_input.txt");
const START_DIR: (i32, i32) = (0, 1);

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
                return (pos.0, pos.1);
            }
        }
        panic!();
    })();

    fn get_direction(char: char, last_dir: (i32, i32)) -> Option<(i32, i32)> {
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

    let mut last_dir: (i32, i32) = START_DIR;
    let mut cursor = (
        (starting_pos.0 as i32 + last_dir.0) as usize,
        (starting_pos.1 as i32 + last_dir.1) as usize,
    );
    let mut step_count = 1;

    loop {
        let char = grid[cursor];
        let dir_to_move = get_direction(char, last_dir).unwrap();

        cursor = (
            (cursor.0 as i32 + dir_to_move.0) as usize,
            (cursor.1 as i32 + dir_to_move.1) as usize,
        );

        last_dir = dir_to_move;

        step_count += 1;

        if cursor == starting_pos {
            break;
        }
    }

    dbg!(step_count / 2);
}

pub fn part2() {
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
                return (pos.0, pos.1);
            }
        }
        panic!();
    })();

    fn get_direction(char: char, last_dir: (i32, i32)) -> Option<(i32, i32)> {
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

    let mut last_dir: (i32, i32) = START_DIR;
    let mut cursor = (
        (starting_pos.0 as i32 + last_dir.0) as usize,
        (starting_pos.1 as i32 + last_dir.1) as usize,
    );
    let mut travelled_positions = Vec::<(f32, f32)>::new();
    travelled_positions.push((starting_pos.0 as f32, starting_pos.1 as f32));
    travelled_positions.push((
        (cursor.0 as f32 / 2.0).floor(),
        (cursor.1 as f32 / 2.0).floor(),
    ));

    loop {
        let char = grid[cursor];
        let dir_to_move = get_direction(char, last_dir).unwrap();

        cursor = (
            (cursor.0 as i32 + dir_to_move.0) as usize,
            (cursor.1 as i32 + dir_to_move.1) as usize,
        );

        last_dir = dir_to_move;

        travelled_positions.push((
            (cursor.0 as f32 / 2.0).floor(),
            (cursor.1 as f32 / 2.0).floor(),
        ));

        if cursor == starting_pos {
            break;
        }
    }

    let polygon = Polygon::new(LineString::from(travelled_positions), vec![]);

    dbg!(polygon.unsigned_area());
}
