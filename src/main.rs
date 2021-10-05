mod bot;
mod config;
mod git_checker;

use std::thread;

fn main(){
    println!("Start");
    let _bot_handler = thread::spawn(||{
        bot::start(config::get_token().to_string()).expect("testing");
    });

    let git_checker_handler = thread::spawn(||{
        git_checker::start();
    });

    // this will guarantee, that script will shut down if git handler catch any update
    let _git_result = git_checker_handler.join();
    // let _bot_result = bot_handler.join();

    println!("HOBA");
}