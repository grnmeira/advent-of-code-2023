use std::fs::File;
use std::io::{self, BufRead};

fn intersects(a: (i64, i64, i64), b: (i64, i64, i64)) -> bool {
  a.0 >= b.0 && a.0 < b.1
}

fn main() {
  let f = File::open("example.txt");
  let b = io::BufReader::new(f.unwrap());
  let b = b.lines();

  let b = b
      .fold(vec![vec![]], |mut v, l| {
          let l = l.unwrap();
          if l.is_empty() {
              v.push(vec![]);
          } else {
              v.last_mut()
                .unwrap()
                .push(l);
          }
          v
      });

 let s = b.first().unwrap();
 let s = s.first().unwrap();
 let (_, s) = s.split_once(':')
   .unwrap();
 let s = s.trim()
   .split_whitespace()
   .map(|n| {
     n.parse::<i64>().unwrap()
   })
   .collect::<Vec<_>>();

  let seeds = s.chunks(2)
    .map(|c|{
      (c[0], c[0]+c[1], 0)
    }).collect::<Vec<_>>();
    
  println!("seeds: {s:?}");

  let maps = b.iter()
    .skip(1)
    .map(|v| {
      let h = v.first().unwrap();
      let m = v.iter()
      .skip(1)
      .map(|l|{
        l.split_whitespace()
        .map(|n| {
          n.parse::<i64>().unwrap()
        }).collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
      (h,m)
    })
    .collect::<Vec<_>>();

  let maps = maps.iter()
    .map(|(_,m)|{
      let mut v = m.iter()
      .map(|m|{
        (m[1], m[1]+m[2], m[1]-m[0])
      }).collect::<Vec<_>>();
      v.sort();
      v
    }).collect::<Vec<_>>();

  println!("maps: {maps:?}");

  for s in seeds {
    let mut merging = vec![s];
    //let mut rv = vec![];
    for m in &maps {
      println!("merging m0: {merging:?} m1: {m:?}");
      for r in &merging {
        let r0 = r.0 + r.2;
        let r1 = r.1 + r.2;
        println!("resolving {r:?} = ({r0},{r1},0)");
        let mut v = vec![r0, r1];
        let mut v = m.iter().fold(v, |mut v, r|{
          v.push(r.0);
          v.push(r.1);
          v
        });
        v.sort();
        let mut v = v.windows(2)
        .filter_map(|w|{
          if w[0] == w[1] {
            return None;
          }
          if w[0] < r0 || w[0] > r1 {
            return None;
          }
          if w[1] < r0 || w[1] > r1 {
            return None;
          }
          Some((w[0], w[1], 0))
        })
        .collect::<Vec<_>>();
        println!("ranges: {v:?}");
        for rr in v.iter_mut(){
          for m in m {
            if intersects(*rr, *m) {
              println!("intersects: {rr:?} {m:?}");
              rr.2 += m.2;
            }
          }
        }
        println!("merged: {merging:?}");
      }
    }
  }
}
