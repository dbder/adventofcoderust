use std::collections::HashSet;

fn main() {
    let answer = include_str!("day03input")
        .lines()
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|sets| find_common_character(sets))
        .map(|c| if c >= 'a' { c as u8 - b'a' + 1 } else { c as u8 - b'A' + 27 })
        .map(|i| i as u32)
        .sum::<u32>();

    println!("{:?}", answer);
}

fn find_common_character(sets: &[HashSet<char>]) -> char {
    return *sets[0]
        .iter()
        .find(|b| sets[1].contains(b) && sets[2].contains(b))
        .unwrap();
}