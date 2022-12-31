#![feature(slice_take)]

use std::collections::VecDeque;

fn main() {
    let chunks = include_str!("day05input")
        .split("\n\n")
        .collect::<Vec<_>>();

    let initial = chunks[0];

    let mut stacks: [VecDeque<char>; 10] = Default::default();

    for line in initial.lines() {
        for j in 0..9 {
            let ch = line.chars().nth((j * 4) + 1);
            if ch.is_none() { continue};
            if ch.unwrap() >= 'A' && ch.unwrap() <= 'Z' {
                stacks[j+1].push_back(ch.unwrap());
            }
        }
    };

    let second = chunks[1];
    for line in second.lines() {

        let mut spl = line.split(" ");
        let mv = spl.nth(1).unwrap().parse::<usize>().unwrap();
        let from = spl.nth(1).unwrap().parse::<usize>().unwrap();
        let to = spl.nth(1).unwrap().parse::<usize>().unwrap();

        for _i in 0..mv {
            let pop = stacks[from].pop_front().unwrap();
            stacks[to].push_front(pop);
        }
    }

    for mut s in stacks {
        if s.is_empty() {continue};
        let answer = &(s.pop_front().unwrap().to_string());
        print!("{}", answer);
    }

}