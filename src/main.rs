mod bot;
mod config;

use std::thread;

fn main(){
    println!("Start");
    let bot_handler = thread::spawn(||{
        bot::start(config::get_token().to_string()).expect("testing");
    });

    let _bot_result = bot_handler.join();

    println!("HOBA");
}