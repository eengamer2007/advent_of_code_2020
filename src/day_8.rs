use crate::help;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Instruction{
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, PartialEq, Clone)]
struct Instr{
    pub instruction: Instruction,
    pub parameter: i32,
    pub executed: bool,
}

impl FromStr for Instr {
    type Err = u8;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        s.trim();
        let x: Vec<&str> = s.split(" ").collect();
        match x[0] {
            "nop" => return Ok(Self{instruction: Instruction::Nop, parameter: 0, executed: false}),
            "acc" => return Ok(Self{instruction: Instruction::Acc, parameter: x[1].parse().unwrap(), executed: false}),
            "jmp" => return Ok(Self{instruction: Instruction::Jmp, parameter: x[1].parse().unwrap(), executed: false}),
            _ => panic!("invalid instruction"),
        }
    }
}

#[allow(dead_code)]
pub fn part_1() -> i32{
    let mut input: Vec<Instr> = help::read("day_8").unwrap();
    let mut acc: i32 = 0;
    let mut prog_count: i64 = 0;
    loop{
        if input[prog_count as usize].executed{
            return acc
        }
        input[prog_count as usize].executed = true;
        let x = &input[prog_count as usize ].instruction;
        println!("{},{},{:?}", acc, prog_count, input[prog_count as usize]);
        match x{
            Instruction::Nop => {},
            Instruction::Jmp => prog_count += (input[prog_count as usize].parameter - 1) as i64,
            Instruction::Acc => acc += input[prog_count as usize].parameter,
        }
        prog_count += 1;
    }
    return acc
}

#[allow(dead_code)]
pub fn part_2() -> i32{
    let mut input: Vec<Instr> = help::read("day_8").unwrap();
    let mut acc: i32 = 0;
    let mut prog_count: i64 = 0;
   'main: for i in 0..input.len() - 1{
        let mut x = input.clone();
        match x[i as usize].instruction{
            Instruction::Acc => continue,
            Instruction::Nop => x.instruction = Instruction::Jmp,
            Instruction::Jmp => x.instruction = Instruction::Nop,
        }
        loop{
            if x[prog_count as usize].executed{
                continue 'main
            }
            x[prog_count as usize].executed = true;
            let x = &x[prog_count as usize ].instruction;
            println!("{},{},{:?}", acc, prog_count, x[prog_count as usize]);
            match y{
                Instruction::Nop => {},
                Instruction::Jmp => prog_count += (x[prog_count as usize].parameter - 1) as i64,
                Instruction::Acc => acc += x[prog_count as usize].parameter,
            }
            prog_count += 1;
        }
    }
    return acc
}