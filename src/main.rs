mod day10;

fn main() {
    let start = std::time::Instant::now();
    day10::part1();
    day10::part2();
    println!("Finished in {:?}", start.elapsed());
}
