
use std::collections::HashSet;
use std::path::Path;

pub fn run() {
  println!("day1");
  let input_path = Path::new("./src/day1/input.txt");
  let file_content = std::fs::read_to_string(input_path).unwrap();

  let file_content_ptr= &file_content;
  part1(file_content_ptr);
  part2(file_content_ptr);
}

fn part1(input: &str) {
  let count:i32 = input
    .lines()
    .map(|x| x.parse::<i32>().unwrap())
    .sum();

  println!("--part1: {:?}", count);
}

fn part2(input: &str) {
  let diff_list:Vec<i32> = input
    .lines()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

  let mut visited_freqs: HashSet<i32> = HashSet::new();
  let mut current_freq = 0;
  visited_freqs.insert(current_freq);

  let mut iteration = 0;
  let diff_list_length = diff_list.len();

  loop {
    let current_diff = diff_list.get(iteration % diff_list_length).unwrap();
    current_freq = current_freq + current_diff;

    if visited_freqs.contains(&current_freq) {
      println!("--part2: {:?}", current_freq);
      return ();
    }
    
    visited_freqs.insert(current_freq);
    iteration = iteration + 1;
  }

}