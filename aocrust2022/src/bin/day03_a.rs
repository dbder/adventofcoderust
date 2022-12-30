fn main() {
    let answer: u32 = include_str!("day03input")
        .lines()
        .map(|l| l.split_at(l.len()/2))
        .map(|l| valueOfCommonCharacter(l) as u32)
        .sum::<u32>();


    println!("{:?}", answer);
}

fn valueOfCommonCharacter(l: (&str, &str)) -> u8 {
    let common = l.0.chars().find(|c| l.1.to_string().contains(*c)).unwrap();
    if common >= 'A' && common <= 'Z' {
        return common as u8 - b'A' + 27;
    }
    return common as u8 - b'a' + 1;

}