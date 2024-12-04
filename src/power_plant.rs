use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_power() {
  let raw_file = load_file("assets/day2_real.txt");
  let parsed_values: Vec<Vec<i32>> = raw_file.iter().map(|line| parse_report(line)).collect();

  let safe_reports = parsed_values.iter().filter(|report| is_safe(&report));
  let safe_skip_reports = parsed_values.iter().filter(|report| is_skipping_report_safe(&report));

  println!("");
  println!("Day 2");
  println!("The number of safe reports is: {}", safe_reports.count());
  println!("The number of safe reports with skips is: {}", safe_skip_reports.count());
}

fn is_skipping_report_safe(report: &Vec<i32>) -> bool {
  for index in 0..(report.len()) {
    let mut skip_report = report.clone();
    skip_report.remove(index);

    if is_safe(&skip_report) {
      return true
    }
  }

  false
}

fn is_safe(report: &Vec<i32>) -> bool {
  let gap_report: Vec<i32> = report.windows(2).map(|x| x[0] - x[1]).collect();

  let safe_increasing = gap_report.iter().all(|x| *x >= 1 && *x <= 3);
  let safe_decreasing = gap_report.iter().all(|x| *x >= -3 && *x <= -1);

  safe_increasing || safe_decreasing
}

fn parse_report(line: &String) -> Vec<i32> {
  let report_values: Vec<&str> = line.split_whitespace().collect();
  
  report_values.iter().map(|value| value.parse().unwrap()).collect()
}

fn load_file(filename: &str) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);

  buf.lines()
      .map(|l| l.expect("Could not parse line"))
      .collect()
}
