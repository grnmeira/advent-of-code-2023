use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum CubeCount {
  Red(usize),
  Green(usize),
  Blue(usize)
}


fn main() {
  // open input.txt as buffered text
    let f = File::open("input.txt");
    let b = io::BufReader::new(f.unwrap()).lines();
  let games = b.map(|line|{
    let l = line.unwrap();
    let (g, c) = l.split_once(':').unwrap();
    let game_id = g.split_once(char::is_whitespace).unwrap().1.parse::<u32>().unwrap();
    let r = c.split(';').collect::<Vec<_>>().iter().map(|r|{
      r.split(',').map(|t|{
        let (n, c) = t.trim().split_once(char::is_whitespace).unwrap();
        let n = n.parse::<usize>().unwrap();
        match c {
          "red" => CubeCount::Red(n),
          "green" => CubeCount::Green(n),
          "blue" => CubeCount::Blue(n),
          _ => panic!("{c}")
        }
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    (game_id, r)
  }).collect::<Vec<_>>();

  // println!("{:?}", games);

  let s = games.iter().filter(|g|{
  	g.1.iter().flatten().all(|c|{
    match c {
      CubeCount::Red(n) if *n > 12 => false,
      CubeCount::Green(n) if *n > 13 => false,
      CubeCount::Blue(n) if *n > 14 => false,
      _ => true
    }
    })
  }).inspect(|g|{
    println!("{:?}", g);
  }).map(|g|{g.0}).sum::<u32>();

  println!("{s:?}");
}