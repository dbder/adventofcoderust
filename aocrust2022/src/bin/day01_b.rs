fn main() {
    let input = include_str!("day01input");

    let mut vec = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    vec.sort_unstable();

    let sum = vec
        .iter()
        .rev()
        .take(3)
        .sum::<i32>();

    println!("{:?}", sum);
}