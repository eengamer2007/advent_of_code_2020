use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::error::Error;
use pretty_env_logger;

pub fn read<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let input = BufReader::new(File::open(format!("./src/inputs/{}.txt",file_path))?);
    Ok(input.lines().map(|v| T::from_str(&v.unwrap()).unwrap()).collect())
}