use pretty_env_logger;
#[macro_use] extern crate log;

mod help;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

#[allow(dead_code)]

fn main() {
    pretty_env_logger::init();
    println!("{}",day_8::part_2());
}