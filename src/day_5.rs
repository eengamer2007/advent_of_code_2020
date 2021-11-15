#[allow(dead_code)]

use crate::help;
use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug)]
struct Seat{
    row: u8,
    column: u8,
    id: u16,
}

impl FromStr for Seat{
    type Err = u8;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        //println!("{}",s);
        let mut row_high: u8 = 127;
        let mut row_low: u8 = 0;
        for i in s[..7].chars(){
            if i == 'F'{
                //print!("F");
                row_high = (row_high + row_low)/2;
            } else {
                //print!("B");
                row_low = (row_high + row_low)/2;
            }
            //println!("{},{}",row_low,row_high)
        }
        //println!("{}",row_high);
        let mut column_high: u8 = 7;
        let mut column_low: u8 = 0;
        for i in s[7..].chars(){
            if i == 'L'{
                //print!("L");
                column_high = (column_high + column_low)/2;
            } else {
                //print!("R");
                column_low = (column_high + column_low)/2;
            }
            //println!("{},{}",column_low,column_high)
        }
        //println!("{}",column_high);
        //println!("");
        Ok(Self{row: row_high, column: column_high, id: ((row_high as u16 * 8) + column_high as u16)})
    }
}

pub fn part_1() -> u16{
    let input = help::read::<Seat>("day_5").unwrap();
    let mut highest: u16 = 0;
    for i in input.iter(){
        if i.id > highest{
            highest = i.id;
        }
        //println!("{:?}", i)
    }
    return highest;
}

pub fn part_2() -> u16{
    let seat_vec: Vec<Seat> = help::read("day_5").unwrap();
    let mut seat_id_vec: Vec<u16> = vec!();
    for i in seat_vec.iter(){
        seat_id_vec.push(i.id)
    }
    seat_id_vec.sort();
    println!("{:?}",seat_id_vec);
    let mut x: u16 = seat_id_vec[0]-1;
    for i in seat_id_vec.iter(){
        if (x+1) != *i{
            return x+1
        }
        x = *i
    }
    0
}