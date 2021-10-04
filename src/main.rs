mod bot;

use std::thread;

fn main(){
    println!("Start");
    let bot_handler = thread::spawn(||{
        bot::main().expect("testing");
    });

    let bot_result = bot_handler.join();

    println!("HOBA");
}