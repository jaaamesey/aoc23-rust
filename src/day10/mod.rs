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
            '-' if last_dir.1 != -1 => Some((0, 1)),
            '-' if last_dir.1 != 1 => Some((0, -1)),

            '|' if last_dir.0 != -1 => Some((1, 0)),
            '|' if last_dir.0 != 1 => Some((-1, 0)),

            'F' if last_dir.1 != -1 => Some((0, 1)),
            'F' if last_dir.0 != -1 => Some((1, 0)),

            '7' if last_dir.1 != 1 => Some((0, -1)),
            '7' if last_dir.0 != -1 => Some((1, 0)),

            'L' if last_dir.1 != -1 => Some((0, 1)),
            'L' if last_dir.0 != 1 => Some((-1, 0)),

            'J' if last_dir.1 != 1 => Some((0, -1)),
            'J' if last_dir.0 != 1 => Some((-1, 0)),
            _ => None,
        }
    }

    const ALL_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut cursor = starting_pos;
    let mut last_dir = (0, 0);
    let mut step_count = 0;

    loop {
        let tuple: Option<((i8, i8), (i32, i32))> = ALL_DIRECTIONS.iter().find_map(|&dir| {
            if dir == (-last_dir.0, -last_dir.1) {
                return None;
            }
            let pos = (cursor.0 + (dir.0 as i32), cursor.1 + (dir.1 as i32));
            if pos.0 < 0 || pos.1 < 0 {
                return None;
            }
            let char = grid.get(pos.0 as usize, pos.1 as usize);

            if char.is_some() {
                let dir = get_direction(*char.unwrap(), last_dir);
                if dir.is_some() {
                    dbg!(char);
                    return Some((dir.unwrap(), pos));
                }
            }
            return None;
        });

        let (next_dir, pos) = tuple.unwrap();
        dbg!(pos);
        cursor = pos;

        last_dir = next_dir;

        step_count += 1;

        if cursor == starting_pos {
            break;
        }
    }

    dbg!(step_count);
}

pub fn part2() {}
