use std::fs::File;
use std::io::{self, BufRead};

fn count(h: &str) -> [usize; 13] {
  let mut t: [usize; 13] = [0; 13];
  for c in h.chars() {
    match c {
      'A' => t[12] += 1,
      'K' => t[11] += 1,
      'Q' => t[10] += 1,
      'J' => t[9] += 1,
      'T' => t[8] += 1,
      '2'..='9' => {
        let n = c.to_digit(10).unwrap() as usize;
        t[n-2] += 1;
      },
      _ => panic!()
    }
  }
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
    let five = c.iter().position(|&n|n==5);
    let  four = c.iter().position(|&n|n==4);
    let triple = c.iter().position(|&n|n==3);
    let pairs = c.iter().enumerate().filter_map(|(i,&c)|if c==2 {
      Some(i)
    } else {
      None
    }).collect::<Vec<_>>();
    let singles = c.iter().enumerate().filter_map(|(i,&c)|if c==1 {
        Some(i)
      } else {
        None
      }).collect::<Vec<_>>();

    let i1 = five.and_then(|i| Some(i+1)).unwrap_or(0);
    let i2 = if let Some(i) = four {
      //i+1
      1
    } else {
      0
    };
    let i3 = if triple.is_some() && pairs.len() == 1 {
      //(triple.unwrap()+1)*100 + pairs[0]+1
      1
    } else {
      0
    };
    let i4 = if triple.is_some() && singles.len() == 2 {
      //triple.unwrap()+1
      1
    } else {
      0
    };
    let i5 = if pairs.len() == 2 {
      //let mut k = pairs[1]+1;
      //k *= 100;
      //k += pairs[0]+1;
      //k
      1
    } else {
      0
    };

    let i6 = if pairs.len() == 1 && singles.len() == 3 {
      //pairs[0]+1
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
        'J' => i7 + 10,
        'T' => i7 + 9,
        '2'..='9' => {
          let n = c.to_digit(10).unwrap() as usize;
          i7 + n-1
        },
        _ => panic!()
      }
    };
    /*for s in singles.iter().rev() {
      i7 = i7*100 + s + 1;
    }*/
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
