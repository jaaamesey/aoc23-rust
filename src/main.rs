mod day05;

fn main() {
    let start = std::time::Instant::now();
    day05::part1();
    day05::part2();
    println!("Finished in {:?}", start.elapsed());
}
