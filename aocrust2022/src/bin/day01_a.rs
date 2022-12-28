fn main() {

    let answer: u32 = include_str!("day01input")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap();

    println!("{:?}", answer);
}