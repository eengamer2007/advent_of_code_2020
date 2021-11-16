use crate::help;
use std::str::FromStr;

struct Color{
    patern: String,
    color: String,
}

struct Bag{
    color: Color,
    contains: Vec<Bag>
}

impl FromStr for Bag{
    type Err = u8;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        s.trim();
        let x: Vec<&str> = s.split(" ").collect();
        let color = Color{patern: x[0].to_string(), color: x[1].to_string()};
        let mut contains: Vec<Bag> = vec!();
        Ok(Self{color, contains})
    }
}

#[allow(dead_code)]
pub fn part_1() -> u16{
    let input: Vec<Bag> = help::read("day_7").unwrap();
    //for i in bag
    0
}

#[allow(dead_code)]
pub fn part_2() -> u16{
    let input: Vec<Bag> = help::read("day_7").unwrap();
    0
}