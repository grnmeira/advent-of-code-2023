use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input.txt");
    let b = io::BufReader::new(f.unwrap()).lines();
  
  let mut v = b.map(|line|{
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
  .map(|(id,w,m)|{
    let mut i = 0u32;
    for n in m {
      if w.contains(&n) {
        i += 1;
      }
    }
    println!("({id},{i})");
    (id,i,1)
  }).collect::<Vec<_>>();
  
  for i in 0..v.len() {
    let (_, p, w) = v[i];
    if p > 0 {
        for j in i+1..i+(p as usize)+1 {
          v[j].2 = v[j].2 + w;
        }
    }
  }

  let r = v.iter()
    .map(|(_,_,w)| w)
    .sum::<u32>();
  
  println!("{r}");
}