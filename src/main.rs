mod day09;

fn main() {
    let start = std::time::Instant::now();
    day09::part1();
    day09::part2();
    println!("Finished in {:?}", start.elapsed());
}
