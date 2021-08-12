use std::{env, io, path::Path,process::Command};
use colorful::{Color, Colorful};

fn main() {

    println!("{esc}{esc}[2J{esc}[1;1H", esc = 27 as char);

    fn generate_status_string(status: bool) -> colorful::core::color_string::CString {

        match status {
            true => {
                return "Good".color(Color::Green3b)
            },
            false => {
                return "Bad".color(Color::Red3a)
            }
        }

    }

    let mut global_dir = env::current_dir().unwrap();
    let mut dir_string = global_dir.to_str().unwrap();
    let mut status = true;

    println!("[Working Dir: {}] [Status: {}]", dir_string.color(Color::Purple1b), generate_status_string(status));

    loop {

        let mut command_data = String::new();
        io::stdin().read_line(&mut command_data).unwrap();
        
        let split_command = command_data.split(' ').collect::<Vec<&str>>();
        let split_command = split_command.iter().map(|e| e.strip_suffix("\r\n").unwrap_or(e) ).collect::<Vec<&str>>();

        match split_command.iter().nth(0) {
            Some(command) => {

                match command.to_lowercase().as_str() {
                    "exit" => {
                        return;
                    }
                    "clear" => {
                        println!("{esc}{esc}[2J{esc}[1;1H", esc = 27 as char);
                        status = true;
                    }
                    "cls" => {
                        println!("{esc}{esc}[2J{esc}[1;1H", esc = 27 as char);
                        status = true;
                    }
                    "cd" => {

                        match split_command.iter().nth(1) {
                            Some(dir) => {

                                match env::set_current_dir(dir) {
                                    Ok(_) => {
                                        global_dir = env::current_dir().unwrap();
                                        dir_string = global_dir.to_str().unwrap();
                                        status = true; 
                                    },
                                    Err(_) => {
                                        status = false; 
                                    },
                                }

                            },
                            None => {
                                println!("Couldn't get the directory to cd into...");
                                status = false;                                
                            },
                        }

                    }
                    _ => {

                        match command.to_lowercase().as_str() {
                            "cargo" => {
                                match Command::new(command_data.trim()).spawn() {
                                    Ok(mut output) => {

                                        output.wait().unwrap();

                                        status = true;
                                    },
                                    Err(err) => {
                                        println!("{err:?}", err=err);
                                        status = false;
                                    },
                                }
                            }   
                            _ => {
                                println!("Unknwon...");
                                status = false;
                            }
                        }
                    }
                }

            },
            None => {
                println!("{} Failed to get commands from list of args...\n~> Args: {:?}", "[ERROR]".color(Color::Red3b), split_command);
                status = false;
            },
        }
         

        println!("[Working Dir: {}] [Status: {}]", dir_string.color(Color::Purple1b),generate_status_string(status));
    }
}
