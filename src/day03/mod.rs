use std::collections::HashMap;

#[derive(Debug)]
struct NumberAtPos { number: i32, row: usize, start_column: usize }

const ADJACENCY_OFFSETS: [(i32, i32); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)];

pub fn part1() {
    fn is_part_symbol(c: char) -> bool {
        !c.is_numeric() && c != '.'
    }
    
    let rows: Vec<Vec<char>> = include_str!("./real_input.txt")
        .split('\n').map(|str| str.chars().collect()).collect();
    
    let mut numbers: Vec<NumberAtPos> = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        let mut curr_str = "".to_string();
        for (x, column) in row.iter().enumerate() {
            if column.is_numeric() {
                curr_str.push(*column);
            }

            if curr_str.len() > 0 && (!column.is_numeric() || x == row.len() - 1) {
                numbers.push(NumberAtPos { 
                    number: curr_str.parse::<i32>().unwrap(),
                    row: y,
                    start_column: x - curr_str.len(),
                });
                curr_str.clear();
            }
        }
    }

    let output = numbers.iter().filter(|num| {
        let num_digits = num.number.to_string().len(); 

        let y = num.row;

        for digit_offset in 0..num_digits {
            let x = num.start_column + digit_offset;
            
            for (check_y, check_x) in ADJACENCY_OFFSETS.iter().filter_map(|(dir_y, dir_x)| {
                let check_y: isize = (y as isize) + (*dir_y as isize);
                let check_x: isize = (x as isize) + (*dir_x as isize);
                if check_x < 0 || check_y < 0 || check_x >= rows[0].len().try_into().unwrap() || check_y >= rows.len().try_into().unwrap() {
                    return None;
                }
                return Some((check_y as usize, check_x as usize));
            }) {
                if is_part_symbol(rows[check_y][check_x]) {
                    return true;
                }
            }
        }

        return false;
    }).fold(0, |acc, curr| curr.number + acc);

    dbg!(output);
}

pub fn part2() {
    let rows: Vec<Vec<char>> = include_str!("./real_input.txt")
        .split('\n').map(|str| str.chars().collect()).collect();
    
    let mut numbers: Vec<NumberAtPos> = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        let mut curr_str = "".to_string();
        for (x, column) in row.iter().enumerate() {
            if column.is_numeric() {
                curr_str.push(*column);
            }

            if curr_str.len() > 0 && (!column.is_numeric() || x == row.len() - 1) {
                numbers.push(NumberAtPos { 
                    number: curr_str.parse::<i32>().unwrap(),
                    row: y,
                    start_column: x - curr_str.len(),
                });
                curr_str.clear();
            }
        }
    }

    let mut gears: HashMap<String, Vec<i32>> = HashMap::new();

    numbers.iter().for_each(|num| {
        let num_digits = num.number.to_string().len(); 

        let y = num.row;

        for digit_offset in 0..num_digits {
            let x = num.start_column + digit_offset;
            
            for (check_y, check_x) in ADJACENCY_OFFSETS.iter().filter_map(|(dir_y, dir_x)| {
                let check_y: isize = (y as isize) + (*dir_y as isize);
                let check_x: isize = (x as isize) + (*dir_x as isize);
                if check_x < 0 || check_y < 0 || check_x >= rows[0].len().try_into().unwrap() || check_y >= rows.len().try_into().unwrap() {
                    return None;
                }
                return Some((check_y as usize, check_x as usize));
            }) {
                let potential_symbol = rows[check_y][check_x];
                if potential_symbol == '*' {
                    let key = format!("{},{}", check_x, check_y);
                    let surrounding_numbers = gears.entry(key).or_insert_with(|| Vec::new());
                    surrounding_numbers.push(num.number);
                    return;
                }
            }
        }
    });

    let output = gears.iter().fold(0, |acc, (_, surrounding_numbers)| {
        if surrounding_numbers.len() != 2 {
            return acc;
        }
        return acc + (surrounding_numbers[0] * surrounding_numbers[1]);
    });

    dbg!(output);
}