use crate::help;

#[allow(dead_code)]
pub fn part_1() -> u16{
    let input: Vec<String> = help::read("day_6").unwrap();
    let mut count: u16 = 0;
    let mut count_vec: Vec<char> = vec!();
    for i in input.iter(){
        let y = i.trim();
        if y == ""{
            count += count_vec.len() as u16;
            count_vec.clear();
        }
        for x in y.chars(){
            if !count_vec.contains(&x){
                count_vec.push(x)
            }
        }
        println!("{}, {}", count, y);
        println!("{:?}, {}", count_vec, count_vec.len());
    }
    return count
}

#[allow(dead_code)]
pub fn part_2() -> u16{
    let input: Vec<String> = help::read("day_6").unwrap();
    let mut count: u16 = 0;
    let mut count_vec: Vec<char> = vec!();
    let mut first = true;
    for i in input.iter(){
        let y = i.trim();
        if y == ""{
            count += count_vec.len() as u16;
            count_vec.clear();
            first = true;
        }else if first{
            count_vec = y.chars().collect();
            first = false;
        } else {
            let mut temp_count_vec: Vec<char> = vec!();
            for x in y.chars(){
                if count_vec.contains(&x){
                    temp_count_vec.push(x)
                }
            }
            count_vec = temp_count_vec;
        }
        println!("{}, {}", count, y);
        count_vec.sort();
        println!("{:?}, {}", count_vec, count_vec.len());
    }
    return count;
}