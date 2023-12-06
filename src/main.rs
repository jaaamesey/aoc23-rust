mod day06;

fn main() {
    let start = std::time::Instant::now();
    day06::part1();
    day06::part2();
    println!("Finished in {:?}", start.elapsed());
}
