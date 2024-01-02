use std::fs::File;
use std::io::{self, BufRead};

fn get_neighbours(v: &Vec<String>, 
                  li: usize,
                  ci: usize,
                  len: usize) -> Vec<[Option<&str>; 3]> {
    let neighbour_lines = [
      li.checked_sub(1).and_then(|i| v.get(i)),
      v.get(li),
      li.checked_add(1).and_then(|i| v.get(i))
    ];

    neighbour_lines.iter()
      .filter(|l| l.is_some())
      .map(|neighbour_line| {
        let line = neighbour_line.unwrap();
        [
          ci.checked_sub(1).and_then(|i| line.get(i..ci)),
          neighbour_line.unwrap().get(ci..ci+len),
          neighbour_line.unwrap().get(ci+len..ci+len+1)
        ]
      }).collect::<Vec<_>>()
}

fn main() {
  let f = File::open("input.txt");
  let b = io::BufReader::new(f.unwrap()).lines();

  let v = b.map(|x| x.unwrap()).collect::<Vec<_>>();

  let mut numbers = Vec::new();
  
  for (li,l) in v.iter()
    .enumerate() {
    let mut number = String::new();
    let mut index: Option<usize> = None;
    println!("{l}");
    for (ci, c) in l.chars().
      enumerate() {

      // If it's a digit, we keep pushing
      if c.is_digit(10) {
        number.push(c);
        if index == None {
          index = Some(ci);
        }
      }

      // Otherwise, we commit to it
      if !number.is_empty() && (!c.is_digit(10) || ci == l.len() - 1) {
        let n_length = number.len();
        let n = number.parse::<i32>().unwrap();
        number.clear();
        if let Some(i) = index {
          // Obtain neighbours of n
          let neighbours = get_neighbours(&v, li, i, n_length);
          if neighbours.iter()
            .flatten()
            .filter_map(|n| *n)
            .any(|n| {
              n.contains(|c: char| {
                !c.is_digit(10) &&
                c != '.'
              })
            }) {
            println!("{n}");
            numbers.push(n);
          }
        }
        index = None;
      }
    } 
  }

  println!("{}", numbers.iter().sum::<i32>());
}
