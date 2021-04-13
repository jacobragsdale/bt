use std::env;
use std::process::Command;
use std::{thread, time};

fn main() {
    //Take in command line arguments
    let args: Vec<String> = env::args().collect();
    let mut command = &String::new();
    
    if args.len() >= 2 {
        command = &args[1];
    }

    //turn on bluetooth and connect to headphones 
    if command == "" || command == "c" {
        Command::new("bluetoothctl")
            .arg("power")
            .arg("on")
            .output()
            .expect("Failed to execute command");

        Command::new("bluetoothctl")
            .arg("connect")
            .arg("4C:87:5D:54:04:81")
            .output()
            .expect("Failed to execute command");
        
        thread::sleep(time::Duration::from_secs(2));

        Command::new("bluetoothctl")
            .arg("connect")
            .arg("4C:87:5D:54:04:81")
            .output()
            .expect("Failed to execute command");
        
    //turn off bluetooth and disconnect from headphones
    } else if command == "d"{
        Command::new("bluetoothctl")
            .arg("power")
            .arg("off")
            .output()
            .expect("Failed to execute command");
    } else {
        println!("Invalid flag. `c` or `d` expected");
    }
}