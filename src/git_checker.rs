use std::process::Command;

pub fn start(){
    println!("Start git checker");
    run_command("chmod +x /home/pi/d_sh_handler_2/src/git_checker/git-repo-watcher");
    run_command("/home/pi/d_sh_handler_2/src/git_checker/git-repo-watcher -d /home/pi/d_sh_handler_2");
}

fn run_command(command:&'static str){
    let output = if cfg!(target_os = "windows") {
        println!("Windows");
        Command::new("cmd")
                .args(["/C", "echo a"])
                .output()
                .expect("failed to execute process")
    } else {
        println!("Not windows");
        Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process")
    };

    println!("{:#?}", output);
}