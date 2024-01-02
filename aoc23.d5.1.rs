use std::fs::File;
use std::io::{self, BufRead};

fn lookup(m: &Vec<Vec<u64>>, v: u64) -> Option<(u64, u64, u64)> {
  let r = m.iter()
  .find(|p| {
    let (d, s, len) = (p[0], p[1], p[2]);
    v >= s && v <= s + len
  });
  if let Some(r) = r {
    Some((r[0],r[1],r[2])) 
  } else {
    None
  }
}

fn map(m: &Vec<Vec<u64>>,
       v: u64) -> u64 {
  if let Some(p) = lookup(m, v) {
    let diff = v - p.1;
    p.0 + diff
  } else {
    v
  }
}

fn main() {
  let f = File::open("input.txt");
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
     n.parse::<u64>().unwrap()
   })
   .collect::<Vec<_>>();

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
          n.parse::<u64>().unwrap()
        }).collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
      (h,m)
    })
    .collect::<Vec<_>>();

  println!("{maps:#?}");

  let r = s.iter()
  .map(|&seed| {
    let seed_to_soil = &maps[0].1;
    let params = lookup(seed_to_soil, seed);
    let mapped = map(seed_to_soil, seed);
    println!("{seed} -> (D,S,L) ({params:?}) -> {mapped:?}");
    let soil_to_fertilizer = &maps[1].1;
    let mapped = map(soil_to_fertilizer, mapped);
    println!("{seed} -> {mapped:?}");
    let fertilizer_to_water = &maps[2].1;
    let mapped = map(fertilizer_to_water, mapped);
    println!("{seed} -> {mapped:?}");
    let water_to_light = &maps[3].1;
    let mapped = map(water_to_light, mapped);
    println!("{seed} -> {mapped:?}");
    let light_to_temperature = &maps[4].1;
    let mapped = map(light_to_temperature, mapped);
    println!("{seed} -> {mapped:?}");
    let temperature_to_humidity = &maps[5].1;
    let mapped = map(temperature_to_humidity, mapped);
    println!("{seed} -> {mapped:?}");
    let humidity_to_location = &maps[6].1;
    let mapped = map(humidity_to_location, mapped);
    println!("{seed} -> {mapped:?}");
    mapped
  })
  .min();

  println!("{r:?}");
}
