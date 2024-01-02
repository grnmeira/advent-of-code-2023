use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::mem::MaybeUninit;

fn main() {
  let f = File::open("input.txt");
  let b = io::BufReader::new(f.unwrap());
  let mut b = b.lines();
  let directions = b.next().unwrap().unwrap();

  let mut map = HashMap::<[char;3],([char;3],[char;3])>::new();

  for l in b.skip(1) {
    let l = l.unwrap();
    let mut orig = unsafe { MaybeUninit::<[char;3]>::uninit().assume_init() };
    let mut ldest = unsafe { MaybeUninit::<[char;3]>::uninit().assume_init() };
    let mut rdest = unsafe { MaybeUninit::<[char;3]>::uninit().assume_init() };
    l[0..3].chars().enumerate().for_each(|(i, c)| orig[i] = c);
    l[7..10].chars().enumerate().for_each(|(i, c)| ldest[i] = c);
    l[12..15].chars().enumerate().for_each(|(i, c)| rdest[i] = c);

    map.insert(orig, (ldest, rdest));
  }

  println!("{directions:?}");
  println!("{map:?}");

  let start = ['A','A','A'];
  let &( mut l, mut r) = map.get(&start).unwrap();
  let mut step = 1;
  for c in directions.chars().cycle() {
    let i = match c {
      'L' => &l,
      'R' => &r,
      _ => panic!()
    };
    if *i == ['Z','Z','Z'] {
      break;
    }
    (l,r) = *map.get(i).unwrap();
    step += 1;
  }
  println!("{step}");
}
