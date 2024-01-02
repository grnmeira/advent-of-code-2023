use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::min;
use std::collections::HashMap;

fn get_neighbours(v: &Vec<String>, 
                  li: usize,
                  ci: usize,
                  len: usize) -> Vec<(usize, usize)> {
    let neighbour_lines = [
      li.checked_sub(1).and_then(|i| v.get(i)),
      v.get(li),
      li.checked_add(1).and_then(|i| v.get(i))
    ];

    neighbour_lines.iter()
  		.enumerate()
      .filter(|(_,l)| l.is_some())
      .map(|(i,l)| {
        let l = l.unwrap();
        let c = ci.saturating_sub(1);
      	let s = l.get(c..min(ci + len + 1,l.len())).unwrap();
        s.match_indices('*')
        .map(|(cj,_)|{
           (li + i - 1, c + cj)
        }).collect::<Vec<_>>()
      })
      .flatten()
  .collect::<Vec<_>>()
}

fn main() {
  let f = File::open("input.txt");
  let b = io::BufReader::new(f.unwrap()).lines();

  let v = b.map(|x| x.unwrap()).collect::<Vec<_>>();
  let mut h = HashMap::new();
  
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
          let nbs = get_neighbours(&v, li, i, n_length);
          println!("{n}: {nbs:?}");
          for nb in nbs {
            h.entry(nb)
              .and_modify(|v: &mut Vec<_>| v.push(n))
              .or_insert(vec![n]);
          }
        }
        index = None;
      }
    } 
  }

  let s = h.iter()
  .inspect(|x| println!("{x:?}"))
  .filter(|(_,v)| v.len() == 2)
  .map(|(_,v)| {
    v.iter().product::<i32>()
  })
  .sum::<i32>();

  println!("{s}");
}
