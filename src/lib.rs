use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn read_one_per_line<T>(file_path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let s = fs::read_to_string(file_path).unwrap();
    s.lines()
        .map(|line| line.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

