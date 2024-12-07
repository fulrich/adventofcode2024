use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

pub fn corrupted_data() {
  println!("");
  println!("Day 3");
  part_one();
  part_two();
}

fn part_one() {
  let computer_code = load_file("assets/day3_real.txt").join("");
  println!("Part one: {}", calculate_multiplications(&computer_code));
}

fn part_two() {
  let mut computer_code = load_file("assets/day3_real.txt").join("");
  computer_code.insert_str(0, "do()");

  let result = computer_code
    .split("don't()")
    .map(|line| line.split("do()").collect::<Vec<&str>>())
    .filter(|split_line| split_line.len() >= 2)
    .map(|split_line| split_line[1..].join(""))
    .map(|do_line| calculate_multiplications(&do_line))
    .reduce(|a, b| a + b)
    .unwrap();

    println!("Part two: {}", result);
}

fn calculate_multiplications(code: &str) -> i64 {
  let regex = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();

  regex.captures_iter(code)
    .map(|captures| {
      let(_, [first, last]) = captures.extract();
      first.parse::<i64>().unwrap() * last.parse::<i64>().unwrap()
    })
    .reduce(|a, b| a + b)
    .unwrap()
}


fn load_file(filename: &str) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);

  buf.lines()
      .map(|l| l.expect("Could not parse line"))
      .collect()
}