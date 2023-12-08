use std::collections::HashMap;

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

pub fn part2() {}
