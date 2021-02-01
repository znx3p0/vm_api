

use std::process::Command;


pub fn sh<'a>(s: &'a str) -> String {

    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
        .args(&["/C", s])
        .output()
        .expect("failed to execute process")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg(s)
        .output()
        .expect("failed to execute process")
    };
    
    let o = std::string::String::from_utf8_lossy(output.stdout.as_slice()).to_string();
    println!("{}", o);
    o

}

pub fn sh_silent<'a>(s: &'a str) {

    
    if cfg!(target_os = "windows") {
        Command::new("cmd")
        .args(&["/C", s])
        .output()
        .expect("failed to execute process")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg(s)
        .output()
        .expect("failed to execute process")
    };

}