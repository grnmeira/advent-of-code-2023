use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input_file).lines();

    let result = lines
        .map(|line| {
            line.unwrap()
                .replace("zero", "z0o")
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|c| c.is_digit(10))
                .fold((None, None), |r, c| match r {
                    (None, None) => (Some(c.to_digit(10).unwrap()), Some(c.to_digit(10).unwrap())),
                    (Some(a), Some(_)) => (Some(a), Some(c.to_digit(10).unwrap())),
                    _ => (None, None),
                })
        })
        .map(|(a, b)| a.unwrap() * 10 + b.unwrap())
        .sum::<u32>();

    println!("{result}");
}
