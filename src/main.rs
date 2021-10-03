use std::thread;
use std::time::Duration;
// mod lib;
mod bot;

fn main(){
    println!("Start");
    bot::main().expect("testing");
    println!("HOBA");
}