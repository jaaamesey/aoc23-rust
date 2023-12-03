mod day03;

fn main() {
    let start = std::time::Instant::now();
    day03::part1();
    day03::part2();
    println!("Finished in {:?}", start.elapsed());
}
