use std::path::Path;
use std::collections::HashMap;

pub fn run() {
  println!("day2");
  let input_path = Path::new("./src/day2/input.txt");
  let file_content = std::fs::read_to_string(input_path).unwrap();

  let file_content_ptr= &file_content;
  part1(file_content_ptr);
  part2(file_content_ptr);
}

fn part1(input: &str) {
  let count: (i32, i32) = input
    .lines()
    .into_iter()
    .map(count_double_and_triples)
    .fold((0, 0), |(d, t), (double, triple)| {
        (d + double, t + triple)
    });

  let result = count.0 * count.1;
  println!("--part1: {:?}", result);
}

fn count_double_and_triples(line: &str) -> (i32, i32) {
  let mut letters_map:HashMap<char, i32> = HashMap::new();

  for ch in line.chars() {
    letters_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
  }

  let mut doubles: i32 = 0;
  let mut triples: i32 = 0;
  let double_marker: i32 = 2;
  let triple_marker: i32 = 3;
  for count in letters_map.values() {
    if *count == double_marker {
      doubles = 1 ;
    }
    if *count == triple_marker {
      triples = 1;
    }
  }

  return (doubles, triples);
}

fn part2(input: &str) {
  let lines: Vec<&str> = input
    .lines()
    .collect();

  let mut visited_list: Vec<&str> = vec![]; 
  for line in lines {

    visited_list
      .iter()
      .for_each(|string_from_visited_list| {
        compare_chars_vec(line, string_from_visited_list)
      }
    );

    visited_list.push(line);

  }
}

fn compare_chars_vec(a: &str, b: &str) {
  let list_a = a.chars();
  let list_b = b.chars();
  let mut diff:i32 = 0;
  for (char_a, char_b) in list_a.zip(list_b) {
    if char_a != char_b {
      diff = diff + 1;
    }
  }

  if diff == 1 {
    let intersection = get_intersection(a, b);
    println!("--part2: {:?}", intersection);
  } 
}

fn get_intersection(a: &str, b: &str) -> String {
  let list_a = a.chars();
  let list_b = b.chars();
  let mut intersection_vec: Vec<char> = vec![];
  for (char_a, char_b) in list_a.zip(list_b) {
    if char_a == char_b {
      intersection_vec.push(char_a);
    }
  }

  let s: String = intersection_vec.into_iter().collect();
  return s;
}