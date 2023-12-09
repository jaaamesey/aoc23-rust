use std::collections::VecDeque;

const INPUT: &str = include_str!("./real_input.txt");

pub fn part1() {
    fn get_diffs(input: &Vec<i64>) -> Vec<i64> {
        let mut output_diffs = Vec::new();
        for i in 1..input.len() {
            let diff = input[i] - input[i - 1].clone();
            output_diffs.push(diff);
        }
        return output_diffs;
    }
    
    let output = INPUT.lines().fold(0, |acc, line| {
        let mut stack = {
            let initial_numbers = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
            let mut stack = Vec::<Vec<i64>>::new();
            stack.push(initial_numbers);
            stack
        };

        while !stack.last().unwrap().iter().all(|&n| n == 0) {
            stack.push(get_diffs(&stack.last().unwrap()));
        }

        (stack.last_mut().unwrap()).push(0);

        while stack.len() > 1 {
            let &last_item = stack.pop().unwrap().last().unwrap();
            let next_line_up = stack.last_mut().unwrap();
            next_line_up.push(next_line_up.last().unwrap() + last_item);
        }

        let &next_in_sequence = stack.last().unwrap().last().unwrap();
        
        return acc + next_in_sequence;
    });

    dbg!(output);
}

pub fn part2() {
    fn get_diffs(input: &VecDeque<i64>) -> VecDeque<i64> {
        let mut output_diffs = VecDeque::new();
        for i in 1..input.len() {
            let diff = input[i] - input[i - 1].clone();
            output_diffs.push_back(diff);
        }
        return output_diffs;
    }
    
    let output = INPUT.lines().fold(0, |acc, line| {
        let mut stack = {
            let initial_numbers = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<VecDeque<_>>();
            let mut stack = Vec::<VecDeque<i64>>::new();
            stack.push(initial_numbers);
            stack
        };

        while !stack.last().unwrap().iter().all(|&n| n == 0) {
            stack.push(get_diffs(&stack.last().unwrap()));
        }

        (stack.last_mut().unwrap()).push_front(0);

        while stack.len() > 1 {
            let &first_item = stack.pop().unwrap().front().unwrap();
            let next_line_up = stack.last_mut().unwrap();
            next_line_up.push_front(next_line_up.front().unwrap() - first_item);
        }

        let &next_in_sequence = stack.last().unwrap().front().unwrap();

        return acc + next_in_sequence;
    });

    dbg!(output);
}

