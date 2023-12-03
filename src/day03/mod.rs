#[derive(Debug)]
struct NumberAtPos { number: i32, row: usize, start_column: usize }

const ADJACENCY_OFFSETS: [(i32, i32); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)];

pub fn part1() {
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
            
            for (adj_offset_y, adj_offset_x) in ADJACENCY_OFFSETS {
                let row = rows.get(y.checked_add_signed(adj_offset_y.try_into().unwrap()).unwrap_or(0));
                
                if row.is_none() {
                    continue;
                }

                let c = row.unwrap().get(x.checked_add_signed(adj_offset_x.try_into().unwrap()).unwrap_or(0)); 
                if c.is_some() && is_part_symbol(*c.unwrap()) {
                    return true;
                }
            }
        }

        return false;
    }).fold(0, |acc, curr| curr.number + acc);

    dbg!(output);
}

fn is_part_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}