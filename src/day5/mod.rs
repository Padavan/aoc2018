use std::{path::Path, cmp::Ordering};


pub fn run() {
  println!("day5");
  let input_path = Path::new("./src/day5/input.txt");
  let file_content = std::fs::read_to_string(input_path).unwrap();

  let file_content_ptr= &file_content;
  let result1 = part1(file_content_ptr);
  println!("part1: {:?}", result1);
  //10564
  let result2 = part2(file_content_ptr);
  println!("part2: {:?}", result2);
}

fn part1(input: &str) -> usize {
  let sequence: Vec<&str> = input
    .split("")
    .collect();
  
  let reacted_sequence = do_reaction(sequence);
  let result:usize = reacted_sequence.len();
  return result;
}

fn do_reaction(sequence: Vec<&str>) -> Vec<&str> {
  let sequence_length = sequence.len();

  let mut links: Vec<usize> = (1..sequence_length).collect();
  let mut index: usize = 0;

  while index < sequence_length - 1 {
    let current_link = links[index];
    if is_triggered(&sequence[index], &sequence[current_link]) {
      links[index] = 0;
      links[current_link] = 0;
      // index = index - 1;
      while links[index] == 0 {
        index = index - 1;
      }
      links[index] = current_link + 1;

    } else {
      index = current_link;
    }
  };

  let sequence2:Vec<&str> = traverse_sequence(sequence, links);
  return sequence2;
}

fn is_triggered(a: &str, b: &str) -> bool {
  if a.to_uppercase().cmp(&b.to_string()) == Ordering::Equal && a.cmp(&b.to_lowercase().to_string()) == Ordering::Equal {
    return true;
  }

  if b.to_uppercase().cmp(&a.to_string()) == Ordering::Equal && b.cmp(&a.to_lowercase().to_string()) == Ordering::Equal {
    return true;
  }

  return false;
}

fn traverse_sequence(seq: Vec<&str>, linking: Vec<usize>) -> Vec<&str> {
  let mut index:usize = 0;

  let mut new_seqence:Vec<&str> = vec![];

  while index < linking.len() {
    index = linking[index];
    let new_char = seq[index];
    if new_char != "" {
      new_seqence.push(new_char);
    }
  }

  return new_seqence;
}

 #[test]
fn is_triggered_test() {
  assert_eq!(is_triggered("a", "A"), true);
  assert_eq!(is_triggered("A", "a"), true);
  assert_eq!(is_triggered("A", "A"), false);
  assert_eq!(is_triggered("a", "a"), false);
  assert_eq!(is_triggered("a", "b"), false);
}

#[test]
fn part1_test() {
  assert_eq!(part1("dabCBAcaDA"), 10);
  assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
  assert_eq!(part1("dabAcCaCBAcCcaDAa"), 9);
  assert_eq!(part1("aBbCcDde"), 2);
  assert_eq!(part1("aAb"), 1);
}

fn part2(input: &str) -> usize {
  let abc = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
  let full_sequence: Vec<&str> = input
    .split("")
    .collect();

  let result = abc
    .iter()
    .map(|letter| {

      let seq_without_letter: Vec<&str> = full_sequence
        .iter()
        .map(|x| x.to_owned())
        .filter(|l| l != letter && l.to_owned() != letter.to_lowercase())
        .collect();

      let reacted_seq:Vec<&str> = do_reaction(seq_without_letter);
      return reacted_seq.len();
    })
    .max_by(|a, b| b.cmp(a)).unwrap();

  return result;
}