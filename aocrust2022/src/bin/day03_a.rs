fn main() {
    let answer: u32 = include_str!("day03input")
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| a.chars().find(|p2| b.to_string().contains(*p2)).unwrap())
        .map(|c| if c >= 'a' { (c as u8 - b'a') as u32 + 1 } else { (c as u8 - b'A') as u32 + 27 })
        .sum::<u32>();

    println!("{:?}", answer);
}