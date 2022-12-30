fn main() {
    let answer = include_str!("day03input")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| value_of_common_character(c) as u32)
        .sum::<u32>();

    println!("{:?}", answer);
}

fn value_of_common_character(c: &[&str]) -> u8 {
    use std::collections::HashSet;

    let mut a: HashSet<char> = HashSet::from_iter(c[0].chars());
    let b: HashSet<char> = HashSet::from_iter(c[1].chars());
    let c: HashSet<char> = HashSet::from_iter(c[2].chars());

    a.retain(|ch| b.contains(ch));
    a.retain(|ch| c.contains(ch));
    let mut common = 'a';
    for x in a { common = x; }

    if common >= 'A' && common <= 'Z' {
        return common as u8 - b'A' + 27;
    }
    return common as u8 - b'a' + 1;
}