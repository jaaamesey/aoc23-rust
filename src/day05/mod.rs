#[derive(Debug)]
struct MapRange {
    source_start: i64,
    destination_start: i64,
    range_length: i64,
}

pub fn part1<'a>() {
    let mut line_iter = include_str!("./real_input.txt")
        .lines()
        .filter(|line| !line.is_empty());
    let seeds = line_iter
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|str| str.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut curr_line = line_iter.next();

    let mut maps = Vec::<Vec<MapRange>>::new();

    while curr_line.is_some() {
        let line = curr_line.unwrap();
        if line.chars().next().unwrap().is_numeric() {
            let mut fields = line
                .split_whitespace()
                .map(|item| item.parse::<i64>().unwrap());

            maps.last_mut().unwrap().push(MapRange {
                destination_start: fields.next().unwrap(),
                source_start: fields.next().unwrap(),
                range_length: fields.next().unwrap(),
            });
        } else {
            maps.push(Vec::new());
        }

        curr_line = line_iter.next();
    }

    let output = seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |acc, map| {
                let applicable_range = map.iter().find(|range| {
                    acc >= range.source_start && acc <= range.source_start + range.range_length
                });
                if applicable_range.is_some() {
                    let range = applicable_range.unwrap();
                    let offset = range.destination_start - range.source_start;
                    return acc + offset;
                }
                return acc;
            })
        })
        .min();

    dbg!(output.unwrap());
}

pub fn part2() {}
