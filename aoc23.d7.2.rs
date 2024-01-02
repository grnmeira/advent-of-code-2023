use std::fs::File;
use std::io::{self, BufRead};

fn count(h: &str) -> [usize; 13] {
  let mut t: [usize; 13] = [0; 13];
  for c in h.chars() {
    match c {
      'A' => t[12] += 1,
      'K' => t[11] += 1,
      'Q' => t[10] += 1,
      'T' => t[9] += 1,
      '2'..='9' => {
        let n = c.to_digit(10).unwrap() as usize;
        t[n-1] += 1;
      },
      'J' => t[0] += 1,
      _ => panic!()
    }
  }
  t
}

fn apply_wildcard(c: &[usize; 13]) -> [usize; 13] {
  if c[0] == 0 {
    return *c;
  }
  let mut t = c.clone();
  	let max = t[1..13].iter().max().unwrap();
  	let p = t.iter().rev().position(|x| x == max).unwrap();
  	t[12-p] += t[0];
    t[0] = 0;
  t
}

fn main() {
  let f = File::open("input.txt");
  let b = io::BufReader::new(f.unwrap());
  let b = b.lines();
  let mut v = b.map(|l|{
      let l = l.unwrap();
      let (h, bid) = l.split_once(char::is_whitespace).unwrap();
      let h = h.to_string();
      (h, bid.parse::<u32>().unwrap())
    }).collect::<Vec<_>>();
  v.sort_by_cached_key(|(h, b)|{
    let c = count(&h);
    let cj = apply_wildcard(&c);
    let five = cj.iter().position(|&n|n==5);
    let  four = cj.iter().position(|&n|n==4);
    let triple = cj.iter().position(|&n|n==3);
    let pairs = cj.iter().enumerate().filter_map(|(i,&c)|if c==2 {
      Some(i)
    } else {
      None
    }).collect::<Vec<_>>();
    let singles = cj.iter().enumerate().filter_map(|(i,&c)|if c==1 {
        Some(i)
      } else {
        None
      }).collect::<Vec<_>>();

    let i1 = if five.is_some() {
      1
    } else {
      0
    };
    
    let i2 = if four.is_some() {
      1
    } else {
      0
    };
    let i3 = if triple.is_some() && pairs.len() == 1 {
      1
    } else {
      0
    };
    let i4 = if triple.is_some() && singles.len() == 2 {
      1
    } else {
      0
    };
    let i5 = if pairs.len() == 2 {
      1
    } else {
      0
    };

    let i6 = if pairs.len() == 1 && singles.len() == 3 {
      1
    } else { 
     0
    };

    let mut i7 = 0;
    for c in h.chars() {
      i7 *= 100;
      i7 = match c {
        'A' => i7 + 13,
        'K' => i7 + 12,
        'Q' => i7 + 11,
        'T' => i7 + 10,
        '2'..='9' => {
          let n = c.to_digit(10).unwrap() as usize;
          i7 + n
        },
        'J' => i7 + 1,
        _ => panic!()
      }
    };
    let r = (i1,i2,i3,i4,i5,i6,i7);
    println!("{h} {r:?}");
    r
  });
  let r = v.iter()
    .enumerate()
    .map(|(i,(_,b))|{
      (i as u32 + 1) * b
    }).sum::<u32>();
 // println!("{v:?}");
  println!("{r}");
}
