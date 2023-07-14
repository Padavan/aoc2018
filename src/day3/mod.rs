use std::path::Path;
use std::collections::HashSet;
use itertools::Itertools;

pub fn run() {
  println!("day3");
  let input_path = Path::new("./src/day3/input.txt");
  let file_content = std::fs::read_to_string(input_path).unwrap();

  let file_content_ptr= &file_content;
  part1(file_content_ptr);
  part2(file_content_ptr);
}

#[derive(Debug)]
struct Claim {
  id: usize,
  x1: usize,
  y1: usize,
  x2: usize,
  y2: usize
}


//98005
fn part1(input: &str) {
  let fabric_size = 1000;

  let mut matrix:Vec<Vec<usize>> = vec![vec![0; fabric_size]; fabric_size];

  input
    .lines()
    .map(parse_claim)
    .for_each(|claim| {
      for col in claim.y1..claim.y2 {
        for row in claim.x1..claim.x2 {
          let current = matrix[col][row];
          matrix[col][row] = current + 1;
        }
      }
    });
  
  let sum = (0..fabric_size)
    .cartesian_product(0..fabric_size)
    .filter(|(col, row)| matrix[*col][*row] > 1)
    .count();

  println!("part1: {:?}", sum);
}

fn parse_claim(line: &str) -> Claim {
  let mut id: usize = 0;
  let mut x1: usize = 0;
  let mut y1: usize = 0;
  let mut diff_x: usize = 0;
  let mut diff_y: usize = 0;

  line
    .split(&['x', ':', ' ', '#', ','])
    .enumerate()
    .for_each(|(idx, part)| {
      match idx {
        1 => id = part.parse::<usize>().unwrap(),
        3 => x1 = part.parse::<usize>().unwrap(),
        4 => y1 = part.parse::<usize>().unwrap(),
        6 => diff_x = part.parse::<usize>().unwrap(),
        7 => diff_y = part.parse::<usize>().unwrap(),
        _ => {},
      }
    });

  let new_claim = Claim {
    id: id.to_owned(),
    x1: x1.to_owned(),
    y1: y1.to_owned(),
    x2: x1 + diff_x,
    y2: y1 + diff_y,
  };

  return new_claim;
}

//331
fn part2(input: &str) {
  let fabric_size = 1000;

  let mut matrix:Vec<Vec<usize>> = vec![vec![0; fabric_size]; fabric_size];
  let mut not_zashkvaret_ids:HashSet<usize> = HashSet::new();

  input
    .lines()
    .map(parse_claim)
    .for_each(|claim| {

      not_zashkvaret_ids.insert(claim.id);
      for col in claim.y1..claim.y2 {
        for row in claim.x1..claim.x2 {
          let current = matrix[col][row];
          if current == 0 {
            matrix[col][row] = claim.id;
          } else {
            not_zashkvaret_ids.remove(&claim.id);
            not_zashkvaret_ids.remove(&current);
          }
        }
      }
    });

  println!("part2: {:?}", not_zashkvaret_ids);
}