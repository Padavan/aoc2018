use std::{path::Path, collections::HashMap };

pub fn run() {
  println!("day7");
  let input_path = Path::new("./src/day7/input-test.txt");
  let file_content = std::fs::read_to_string(input_path).unwrap();

  let file_content_ptr= &file_content;
  let result1 = part1(file_content_ptr);
  println!("--part1: {:?}", result1);

  let result2 = part2(file_content_ptr);
  println!("--part2: {:?}", result2);
}

static abc:Vec<&str> = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

fn part1(input: &str) -> String {

  let mut map:HashMap<&str, Vec<&str>> = HashMap::new();

  let orders:Vec<(&str, &str)> = input.lines().map(parse_input).collect();

  println!("{:?}", orders);

  return "string".to_owned();
}

fn parse_input(line: &str) -> (&str, &str) {
  let mut order: (&str, &str) = ("", "");

  line
    .split(" ")
    .enumerate()
    .for_each(|(idx, part)| {
      match idx {
        1 => order.0 = part,
        7 => order.1 = part,
        _ => {},
      }
    });

  return order;
}

#[test]
fn parse_input_test() {
  let mock_input = "Step F must be finished before step E can begin.";
  let result = parse_input(&mock_input);
  assert_eq!(result, ("F", "E"));
}

fn part2(input: &str) -> String {
  return "string".to_owned();
}