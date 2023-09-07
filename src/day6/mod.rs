use std::{path::Path, collections::HashMap};

pub fn run() {
  println!("day6");
  let input_path = Path::new("./src/day6/input.txt");
  let file_content = std::fs::read_to_string(input_path).unwrap();

  let file_content_ptr= &file_content;
  let result1 = part1(file_content_ptr);
  println!("--part1: {:?}", result1);
  // 4171
  let result2 = part2(file_content_ptr);
  println!("--part2: {:?}", result2);
}

#[derive(Debug, Copy, Clone)]
struct Point {
  id: usize,
  x: i32,
  y: i32,
}

impl Point {
  fn get_distance(&self, x: i32, y: i32) -> i32 {
    return i32::abs(self.x - x) + i32::abs(self.y - y);
  }
}

fn part1(input: &str) -> usize {
  let initial_points:Vec<Point> = input
    .lines()
    .enumerate()
    .map(|(idx, line)| {
      let parts: Vec<&str> = line.split(", ").collect();
      if parts.len() > 1 {
        return Point{id: idx + 1, x: parts[0].parse::<i32>().unwrap(), y: parts[1].parse::<i32>().unwrap() }
      } else {
        unreachable!("coordinates should be provided by input")
      }
    })
    .collect();

  let mut min_x: i32 = initial_points[0].x;
  let mut min_y: i32 = initial_points[0].y;
  let mut max_x: i32 = initial_points[0].x;
  let mut max_y: i32 = initial_points[0].y;

  for p in &initial_points {
    if p.x > max_x {
      max_x = p.x;
    }
    if p.x < min_x {
      min_x = p.x
    }
    if p.y > max_y {
      max_y = p.y;
    }
    if p.y < min_y {
      min_y = p.y
    }
  };

  let columns_number = max_x - min_x ;
  let rows_number = max_y - min_y ;

  let row = vec![0; usize::try_from(columns_number).unwrap()];
  let mut matrix = vec![row; usize::try_from(rows_number).unwrap()];


  for y in min_y..max_y {
    for x in min_x..max_x {
      let id = get_id(x, y, &initial_points);

      let ycoord = usize::try_from(y - min_y).unwrap();
      let xcoord = usize::try_from(x - min_x).unwrap();
      matrix[ycoord][xcoord] = id;
    }
  }

  let stats = get_stats(&matrix);

  let area: usize = stats
    .iter()
    .filter(|(_k, v)| **v < 10000)
    .max_by(|a, b| a.1.cmp(&b.1))
    .map(|(_k, v)| *v)
    .unwrap();

  return area;
}

fn get_stats(matrix: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
  let mut stats:HashMap<usize, usize> = HashMap::new();
  let last_row = matrix.len() - 1;
  let last_column = matrix[0].len() - 1;

  matrix.iter().enumerate().for_each(|(row_id, row)| {
    row.iter().enumerate().for_each(|(column_id, cell)| {
      *stats.entry(*cell).or_insert(0) += 1;
      if column_id == 0 || row_id == 0 || column_id == last_column || row_id == last_row {
        *stats.entry(*cell).or_insert(0) += 10000;
      }
    })
  });

  return stats;
}

fn get_id(x: i32, y: i32, points: &Vec<Point>) -> usize {
  let mut minimal_distance = 10000;
  let mut point_id = 100;

  for point in points {
    let distance: i32 = point.get_distance(x, y);
    if minimal_distance == distance {
      point_id = 0;
    }

    if minimal_distance > distance {
      // println!("minimal > {:?}", distance);
      minimal_distance = distance;
      point_id = point.id;
    }
  }

  return point_id;
}

fn part2(input: &str) -> usize {
  let initial_points:Vec<Point> = input
    .lines()
    .enumerate()
    .map(|(idx, line)| {
      let parts: Vec<&str> = line.split(", ").collect();
      if parts.len() > 1 {
        return Point{id: idx + 1, x: parts[0].parse::<i32>().unwrap(), y: parts[1].parse::<i32>().unwrap() }
      } else {
        unreachable!("coordinates should be provided by input")
      }
    })
    .collect();


  let mut min_x: i32 = initial_points[0].x;
  let mut min_y: i32 = initial_points[0].y;
  let mut max_x: i32 = initial_points[0].x;
  let mut max_y: i32 = initial_points[0].y;

  for p in &initial_points {
    if p.x > max_x {
      max_x = p.x;
    }
    if p.x < min_x {
      min_x = p.x
    }
    if p.y > max_y {
      max_y = p.y;
    }
    if p.y < min_y {
      min_y = p.y
    }
  };

  let columns_number = max_x - min_x ;
  let rows_number = max_y - min_y ;

  let row = vec!["."; usize::try_from(columns_number).unwrap()];
  let mut matrix = vec![row; usize::try_from(rows_number).unwrap()];

  let max_distance = 10000;

  for y in min_y..max_y {
    for x in min_x..max_x {
      let mut sum_distance = 0;

      initial_points.iter().for_each(|p| {
        let distance_for_point = i32::abs(p.x - x) + i32::abs(p.y - y);
        sum_distance += distance_for_point;
      });

      if sum_distance < max_distance {
        let ycoord = usize::try_from(y - min_y).unwrap();
        let xcoord = usize::try_from(x - min_x).unwrap();
        matrix[ycoord][xcoord] = "#";
      };
    }
  }

  let area = matrix
    .iter()
    .map(|row| row
        .iter()
        .filter(|x| *x == &"#")
        .count()

      ).sum();

  return area;
}