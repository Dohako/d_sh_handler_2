mod bot;
mod config;
mod git_checker;

use std::thread;

fn main(){
    println!("Start main.rs");
    let _bot_handler = thread::spawn(||loop{
        let bot_output = bot::start(config::get_token().to_string());
    });

    let git_checker_handler = thread::spawn(||{
        git_checker::start();
    });

    // this will guarantee, that script will shut down if git handler catch any update
    let _git_result = git_checker_handler.join();
    // let _bot_result = _bot_handler.join();
    // println!("{:#?}", bot_output);
    println!("{:#?}", _git_result);

    println!("HOBA");
}