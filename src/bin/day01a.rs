use aoc_2021::day01::calculate_increase;

fn main() {
    let raw_measurements = include_str!("../../data/day01a.txt");

    let measurements = raw_measurements
        .trim()
        .split('\n')
        .filter_map(|m| str::parse::<usize>(m).ok())
        .collect();

    let increase = calculate_increase(measurements);

    println!("{}", increase);
}
