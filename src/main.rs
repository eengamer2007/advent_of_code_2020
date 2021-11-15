use pretty_env_logger;
#[macro_use] extern crate log;

mod help;
mod day_4;
mod day_5;
mod day_6;

#[allow(dead_code)]

fn main() {
    pretty_env_logger::init();
    println!("{}",day_6::part_1());
}