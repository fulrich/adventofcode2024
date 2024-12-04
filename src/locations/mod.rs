use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

pub fn part_one() {
  let file = "assets/1_1_real.txt";
  let (first_list, second_list) = get_lists(file).unwrap();

  let zipped_list = first_list.iter().zip(second_list.iter());
  let mut difference_count = 0;
  for (first, second) in zipped_list {
      difference_count += first.abs_diff(*second);
  }

  println!("Difference is: {}", difference_count);
}

pub fn part_two() {
  let file = "assets/1_1_real.txt";
  let (first_list, second_list) = get_lists(file).unwrap();

  let mut similarity_score = 0;
  for first in first_list {
      let count: i32 = second_list.iter().filter(|x| **x == first).count().try_into().unwrap();
      similarity_score += first * count;
  }
  println!("Similarity score is: {}", similarity_score);
}

fn get_lists(file: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
  let file = File::open(file)?;
  let reader = BufReader::new(file);

  let mut first_list: Vec<i32> = Vec::new();
  let mut second_list: Vec<i32> = Vec::new();

  for line in reader.lines() {
      let single_line = line?;
      let parts: Vec<&str> = single_line.split_whitespace().collect();

      first_list.push(parts.first().unwrap().parse()?);
      second_list.push(parts.last().unwrap().parse()?);
  }

  first_list.sort();
  second_list.sort();

  Ok((first_list, second_list))
}