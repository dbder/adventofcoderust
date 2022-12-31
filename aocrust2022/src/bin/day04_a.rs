fn main() {
    let answer = include_str!("day04input")
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(a, b)| (a.split_once("-").unwrap(), b.split_once("-").unwrap()))
        .map(|(a, b)| ((a.0.to_string().parse::<u32>().unwrap()..=a.1.to_string().parse::<u32>().unwrap()), (b.0.to_string().parse::<u32>().unwrap()..=b.1.to_string().parse::<u32>().unwrap())))
        .filter(|(a, b)| ((a.contains(&b.start()) && a.contains(&b.end()))) || ((b.contains(&a.start()) && b.contains(&a.end()))))
        .count();
    // .collect::<Vec<_>>();


    println!("{:?}", answer);
}