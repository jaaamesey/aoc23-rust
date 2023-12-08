use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./real_input.txt");

struct Node {
    id: String,
    left_id: String,
    right_id: String,
}

pub fn part1() {
    let mut lines_iter = INPUT.lines();
    let instructions = lines_iter.next().unwrap();
    lines_iter.next();
    let nodes: HashMap<String, Node> = lines_iter.fold(HashMap::new(), |mut map, line| {
        let mut line_str = line.to_string();
        line_str.retain(|c| !"()=,".contains(c));
        let mut field_iter = line_str.split_whitespace();
        let id = field_iter.next().unwrap().to_string();
        let left_id = field_iter.next().unwrap().to_string();
        let right_id = field_iter.next().unwrap().to_string();
        map.insert(
            id.clone(),
            Node {
                id,
                left_id,
                right_id,
            },
        );
        map
    });

    let mut num_steps = 0;
    let mut curr_node = nodes.get("AAA").unwrap();
    while curr_node.id != "ZZZ" {
        for instruction in instructions.chars() {
            num_steps += 1;
            curr_node = match instruction {
                'L' => nodes.get(&curr_node.left_id).unwrap(),
                'R' => nodes.get(&curr_node.right_id).unwrap(),
                _ => panic!(),
            };
            if curr_node.id == "ZZZ" {
                break;
            }
        }
    }

    dbg!(num_steps);
}

pub fn part2() {
    let mut lines_iter = INPUT.lines();
    let instructions = lines_iter.next().unwrap();
    lines_iter.next();
    let nodes: HashMap<String, Node> = lines_iter.fold(HashMap::new(), |mut map, line| {
        let mut line_str = line.to_string();
        line_str.retain(|c| !"()=,".contains(c));
        let mut field_iter = line_str.split_whitespace();
        let id = field_iter.next().unwrap().to_string();
        let left_id = field_iter.next().unwrap().to_string();
        let right_id = field_iter.next().unwrap().to_string();
        map.insert(
            id.clone(),
            Node {
                id,
                left_id,
                right_id,
            },
        );
        map
    });

    let mut num_steps = 0;
    let mut curr_nodes = nodes
        .values()
        .filter(|node| node.id.ends_with('A'))
        .collect::<Vec<_>>();

    // kudos to andrew for the idea of getting the lcms of loops
    let mut seen_ids = HashMap::<String, HashSet<String>>::new();
    let mut loop_starts = HashMap::<String, i64>::new();
    let mut loop_sizes = HashMap::<String, i64>::new();

    while loop_sizes.len() < curr_nodes.len() {
        for instruction in instructions.chars() {
            num_steps += 1;

            curr_nodes.iter().for_each(|node| {
                if node.id.ends_with('Z')
                    && loop_starts.contains_key(&node.id)
                    && !loop_sizes.contains_key(&node.id)
                {
                    loop_sizes.insert(
                        node.id.clone(),
                        num_steps - loop_starts.get(&node.id).unwrap(),
                    );
                }

                if node.id.ends_with('Z')
                    && !loop_starts.contains_key(&node.id)
                    && !(seen_ids
                        .entry(node.id.clone())
                        .or_insert(HashSet::new())
                        .insert(node.id.clone()))
                {
                    // Found loop - record steps at start
                    loop_starts.insert(node.id.clone(), num_steps);
                }
            });

            curr_nodes = curr_nodes
                .iter()
                .map(|node| match instruction {
                    'L' => nodes.get(&node.left_id).unwrap(),
                    'R' => nodes.get(&node.right_id).unwrap(),
                    _ => panic!(),
                })
                .collect();
        }
    }
    dbg!(loop_sizes.values().fold(1, |acc, curr| lcm(acc, *curr)));
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
