mod day08;

fn main() {
    let start = std::time::Instant::now();
    day08::part1();
    day08::part2();
    println!("Finished in {:?}", start.elapsed());
}
