use std::{path::Path, collections::HashMap};


pub fn run() {
  println!("day4");
  let input_path = Path::new("./src/day4/input.txt");
  let file_content = std::fs::read_to_string(input_path).unwrap();

  let file_content_ptr= &file_content;
  common_process(file_content_ptr);
}

#[derive(Debug)]
struct Shift {
  id: String,
  records: Vec<usize>,
}

fn parse_line(line: &str) -> Vec<String> {
  let parts:String = line.replace('[', "");
  let parts:String = parts.replace(']', "");
  let parts:Vec<String> = parts.split(" ").map(|x| x.to_owned()).collect();

  return parts;
}

fn common_process(input: &str) {
  let mut input_lines: Vec<&str> = input
    .lines()
    .collect();
  input_lines.sort();


  let mut shift_arr:Vec<Shift> = vec![];

  input_lines
    .iter()
    .map(|x| parse_line(x))
    .for_each(|line_parts| {
      let time = line_parts.get(1).unwrap();
      let marker: &str = line_parts.get(2).unwrap();

      match marker {
        "Guard" => {
          let cur_guard = line_parts.get(3).unwrap();
          let new_shift: Shift = Shift{id: cur_guard.to_owned(), records: vec![0; 60]};
          shift_arr.push(new_shift);
        },
        "falls" => {
          let last_shift:&mut Shift = shift_arr.last_mut().unwrap();
          let minutes = time.split(":").last().unwrap().parse::<usize>().unwrap();

          let modified_records: Vec<usize> = last_shift.records
            .iter()
            .enumerate()
            .map(|(idx, x)| if idx >= minutes { 1 } else { x.to_owned() })
            .collect();

          *last_shift = Shift{id: last_shift.id.to_owned(), records: modified_records};
        },
        "wakes" => {
          let last_shift:&mut Shift = shift_arr.last_mut().unwrap();
          let minutes = time.split(":").last().unwrap().parse::<usize>().unwrap();

          let modified_records: Vec<usize> = last_shift.records
            .iter()
            .enumerate()
            .map(|(idx, x)| if idx >= minutes { 0 } else { x.to_owned() })
            .collect();

          *last_shift = Shift{id: last_shift.id.to_owned(), records: modified_records};
        },
        _ => {}
      }
    });

  part1(&shift_arr);
  part2(&shift_arr);
}

fn part1(shift_arr: &Vec<Shift>) {

    let mut guard_stats:HashMap<&str, Vec<usize>> = HashMap::new();

    shift_arr.iter().for_each(|shift| {
      guard_stats
        .entry(&shift.id)
        .and_modify(|curr_state| {
          *curr_state = curr_state.iter().enumerate().map(|(idx, x)| shift.records[idx] + x).collect();
        })
        .or_insert(shift.records.to_owned());
    });

    let max_guard_id = guard_stats
      .iter()
      .max_by(|a,b| {
        let a_sum:usize = a.1.iter().sum();
        let b_sum:usize = b.1.iter().sum();
        return a_sum.cmp(&b_sum);
      })
      .map(|(k,_v)| k);


    let max_guard_id_number:usize = max_guard_id.unwrap().replace("#", "").parse::<usize>().unwrap();

    let sleepy_minute = guard_stats.get(max_guard_id.unwrap()).unwrap()
      .iter()
      .enumerate()
      .max_by(|(_, a), (_, b)| a.cmp(&b))
      .map(|(index, _)| index);


  println!("part1: {:?}", max_guard_id_number * sleepy_minute.unwrap());
  // 48680
  // 18 unwraps
}

fn part2(shift_arr: &Vec<Shift>) {

    let mut guard_stats:HashMap<&str, Vec<usize>> = HashMap::new();

    shift_arr.iter().for_each(|shift| {
      guard_stats
        .entry(&shift.id)
        .and_modify(|curr_state| {
          *curr_state = curr_state.iter().enumerate().map(|(idx, x)| shift.records[idx] + x).collect();
        })
        .or_insert(shift.records.to_owned());
    });

    let max_minutes_guard_id = guard_stats
      .iter()
      .max_by(|a, b| {
        let a_max:&usize = a.1.iter().max().unwrap();
        let b_max:&usize = b.1.iter().max().unwrap();
        return a_max.cmp(&b_max);
      })
      .map(|(k,_v)| k).unwrap();

  let max_guard_id_number:usize = max_minutes_guard_id.replace("#", "").parse::<usize>().unwrap();

    let sleepy_minute = guard_stats.get(max_minutes_guard_id).unwrap()
      .iter()
      .enumerate()
      .max_by(|(_, a), (_, b)| a.cmp(&b))
      .map(|(index, _)| index);


  println!("part2: {:?}", max_guard_id_number * sleepy_minute.unwrap());
  // 94826
}