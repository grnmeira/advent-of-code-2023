use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input.txt");
    let b = io::BufReader::new(f.unwrap()).lines();
  
  let r = b.map(|line|{
    let l = line.unwrap();
    let l = l.split_once(':');
    let (id,g) = l.unwrap();
    let id = id.split_once(
      char::is_whitespace);
    let (_,id) = id.unwrap();
    let id = id.trim().parse::<usize>();
    let id = id.unwrap();
    let g = g.split_once('|');
    let (w,m) = g.unwrap();
    let w = w.split_whitespace()
      .map(|n|n.parse::<u32>().unwrap())
      .collect::<Vec<_>>();
    let m = m.split_whitespace()
      .map(|n|n.trim().parse::<u32>().unwrap())
      .collect::<Vec<_>>();
    (id,w,m)
  }).inspect(|i|println!("{i:?}"))
  .map(|(_,w,m)|{
    let mut i = 0u32;
    for n in m {
      if w.contains(&n) {
        i += 1;
      }
    }
    if i > 0 { 
      return 2u32.pow(i - 1);
    }
    return 0;
  }).sum::<u32>();
  println!("{r}");
}