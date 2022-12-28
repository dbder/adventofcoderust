fn main() {
    let result = include_str!("day02input")
        .lines()
        .map(|s| to_score(s))
        .sum::<i32>();

    println!("{}", result);
}

// RPS to score: ABC for comp, XYZ for player. X = lose, Y = draw, Z = win
//  example : comp:C player:Z  Z=win C=Scissor Player must pick Rock.
//  Rock = 1,  winning = 6. return 7
fn to_score(s: &str) -> i32 {
    let a = s.chars().nth(0).unwrap() as i32 - 'A' as i32;
    let b = s.chars().nth(2).unwrap() as i32 - 'X' as i32;

    if b == 0 { return ((a + 3 - 1) % 3) + 1; };
    if b == 1 { return 3 + a + 1; }

    return 6 + ((a + 3 + 1) % 3) + 1;
}