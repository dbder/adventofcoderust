
fn main() {
    let result  = include_str!("day02input")
        .lines()
        .map(|s| to_score(s))
        .sum::<i32>();

    println!("{}", result);

}

fn to_score(s: &str) -> i32 {
    let a = s.chars().nth(0).unwrap() as i32 - 65;
    let b = s.chars().nth(2).unwrap() as i32 - 88;

    if a == b { return 3 + b + 1; };
    if a == (b + 1) % 3 { return b + 1; }
    return 6 + b + 1;
}