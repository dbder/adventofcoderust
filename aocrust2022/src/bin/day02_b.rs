fn main() {
    let result = include_str!("day02input")
        .lines()
        .map(|t| t.as_bytes())
        .map(|s| to_score(s) as u32)
        .sum::<u32>();

    println!("{}", result);
}

fn to_score(s: &[u8]) -> u8 {
    println!("{:?}", s);
    let a = s[0] - b'A';
    let b = s[2] - b'X';

    if b == 0 { return ((a + 3 - 1) % 3) + 1; };
    if b == 1 { return 3 + a + 1; }

    return 6 + ((a + 3 + 1) % 3) + 1;
}