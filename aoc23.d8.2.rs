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

  let mut state = vec![];

  for l in b.skip(1) {
    let l = l.unwrap();
    let mut orig = unsafe { MaybeUninit::<[char;3]>::uninit().assume_init() };
    let mut ldest = unsafe { MaybeUninit::<[char;3]>::uninit().assume_init() };
    let mut rdest = unsafe { MaybeUninit::<[char;3]>::uninit().assume_init() };
    l[0..3].chars().enumerate().for_each(|(i, c)| orig[i] = c);
    l[7..10].chars().enumerate().for_each(|(i, c)| ldest[i] = c);
    l[12..15].chars().enumerate().for_each(|(i, c)| rdest[i] = c);

    map.insert(orig, (ldest, rdest));

    if orig[2] == 'A' {
      state.push(orig);
    }
  }

  println!("{state:?}");

  let mut step = 0;
  for c in directions.chars().cycle() {
    if state.iter().all(|s| s[2] == 'Z') {
      break;
    }
    state.iter_mut()
    .enumerate()
    .for_each(|(i, s)| {
      if s[2] == 'Z' {
				println!("{i} {s:?} {step}");
      }
      let (l,r) = map.get(s).unwrap();
      if c == 'L' {
        *s = *l;
      } else {
        *s = *r;
      }
    });
    step += 1;
  }
  println!("{state:?}");
  println!("{step}");
}
