fn main() {
    let result = include_str!("day02input")
        .lines()
        .map(|s| s.as_bytes())
        .map(|s| to_score(s) as u32)
        .sum::<u32>();

    println!("{}", result);
}

fn to_score(s: &[u8]) -> u8 {
    let a = s[0] - b'A';
    let b = s[2] - b'X';

    if a == b { return 3 + b + 1; };
    if a == (b + 1) % 3 { return b + 1; }
    return 6 + b + 1;
}