use pretty_env_logger;
#[macro_use] extern crate log;

mod help;
mod day_4;
mod day_5;

fn main() {
    pretty_env_logger::init();
    println!("{}",day_5::part_2());
}