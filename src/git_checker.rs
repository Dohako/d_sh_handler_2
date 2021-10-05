use std::process::Command;

pub fn start(){
    println!("Start git checker!");
    run_command("chmod +x /home/pi/d_sh_handler_2/git_checker/git-repo-checker");
    let out = run_command("/home/pi/d_sh_handler_2/git_checker/git-repo-checker");
    let s = format!("{:?}", &out);
    println!("{}", s);
    println!("{:?}", out);
}

fn run_command(command:&'static str) -> Vec<u8>{
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
    let _result = output.stdout;
    let _err_result = output.stderr;
    return _err_result;
}