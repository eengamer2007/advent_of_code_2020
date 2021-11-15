#[allow(dead_code)]

use crate::help::read;
use std::str::FromStr;
use pretty_env_logger;

pub fn part_1(){
    let input = read::<String>("day_4").unwrap();
    let mut correct: u32 = 0;
    let mut clear_count: u32 = 0;
    let mut current_vec: Vec<&str> = vec!();
    for  i in input.iter() {
        let trimed = i.as_str().trim();
        //println!("{:?}", trimed);
        if trimed != "" {
            for x in trimed.split(" "){
                //debug!("{:?}", x);
                current_vec.push(x);
            }
        } else{
            trace!("{}", current_vec.len());
            if current_vec.len() == 8{
                println!("correct");
                correct += 1;
            } else if current_vec.len() == 7  {
                let mut add = 1;
                for trimed in current_vec.iter() {
                    if &trimed[..=2] == "cid" {
                        debug!("cid");
                        add = 0;
                        break;
                    }
                }
                correct += add;
                if add == 1{
                    println!("correct");
                } else {
                    println!("false");
                    current_vec.clear();
                }
            }
            println!("{}", correct);
            clear_count += 1;
            current_vec.clear();
        }
    }
    //println!("{:?}, {}", input, correct);
    println!("{}, {}", clear_count, correct);
}

enum Height{
    In(u16),
    Cm(u16)
}

impl FromStr for Height{
    type Err = u8;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let mut out: Self = Height::Cm(0);
        match &s[s.len()-2..]{
            "in" => out = Height::In(s[..s.len()-2].parse().unwrap()),
            "cm" => out = Height::Cm(s[..s.len()-2].parse().unwrap()),
            _ => {}
        }
        Ok(out)
    }
}

struct Passport{
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: u32,
}
/*
impl Passport{
    fn gen_pass(input: Vec<&str>) -> Result<Self, u8>{
        
        for i in input.iter(){
            let x = i.split(":").collect();
            match x[0]
        }
        Self{}
    }
}
*/
pub fn part_2(){
    let input = read::<String>("day_4").unwrap();
    let mut current_vec: Vec<&str> = vec!();
    for  i in input.iter() {
        let trimed = i.as_str().trim();
        //println!("{:?}", trimed);
        if trimed != "" {
            for x in trimed.split(" "){
                //debug!("{:?}", x);
                current_vec.push(x);
            }
        } else{
            /*
            match Passport::gen_pass(current_vec){
                Ok(_) => {},
                Err(_) => {},
            }
            */
            current_vec.clear()
        }
    }
}