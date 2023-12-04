mod day04;

fn main() {
    let start = std::time::Instant::now();
    day04::part1();
    day04::part2();
    println!("Finished in {:?}", start.elapsed());
}
