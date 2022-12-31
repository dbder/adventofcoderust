use std::ops::RangeInclusive;

fn main() {
    let answer = include_str!("day04input")
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(a, b)| (a.split_once("-").unwrap(), b.split_once("-").unwrap()))
        .map(|(a, b)| (to_range(a), to_range(b)))
        .filter(|(a, b)| a.start() <= b.end() && a.end() >= b.start())
        .count();

    println!("{:?}", answer);

}

fn to_range(t: (&str, &str)) -> RangeInclusive<u32> {
    return t.0.parse::<u32>().unwrap()..=t.1.parse::<u32>().unwrap();
}