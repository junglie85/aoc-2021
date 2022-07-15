use aoc_2021::day01::calculate_increase_windowed;

fn main() {
    let raw_measurements = include_str!("../../data/day01a.txt");

    let measurements = raw_measurements
        .trim()
        .split('\n')
        .filter_map(|m| str::parse::<usize>(m).ok())
        .collect();

    let increase = calculate_increase_windowed(measurements, 3);

    println!("{}", increase);
}
