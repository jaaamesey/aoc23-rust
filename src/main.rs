mod day07;

fn main() {
    let start = std::time::Instant::now();
    day07::part1();
    day07::part2();
    println!("Finished in {:?}", start.elapsed());
}
