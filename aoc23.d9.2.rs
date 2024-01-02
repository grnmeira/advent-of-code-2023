use std::fs::File;
use std::io::{self, BufRead};
use std::rc::Rc;

fn main() {
  let f = File::open("input.txt");
  let b = io::BufReader::new(f.unwrap());
  let b = b.lines();
  let r = b.map(|l|{
    let l = l.unwrap();
    let l = l.split(" ")
      .map(|s| s.parse::<i32>().unwrap())
      .collect::<Vec<_>>();
    let mut l = Rc::new(l);
    let first = l[0];
    let mut v = vec![];
    loop {
    	let d = l.windows(2)
      .map(|w| w[1] - w[0])
      .collect::<Vec<_>>();
      let d = Rc::new(d);
      if d.iter().all(|&x| x == 0){
        break;
      }
      v.push(d.clone());
      l = d.clone();
    }
    let mut p = vec![0];
    for d in v.iter().rev() {
      let first = d[0];
      p.push(first - *p.last().unwrap());
    }
    println!("{:?}", p);
    first - *p.last().unwrap()
  })
  .collect::<Vec<_>>();
  let r: i32 = r.iter().sum();
  println!("{r:?}");
}
