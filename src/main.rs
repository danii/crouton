use colorful::{Color, Colorful};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::ptr::read;
use git2::Repository;
use std::time::{Duration, SystemTime};
use std::{
    fs:: {read_dir},
    env,
    io::{self, Write},
    process::Command,
};

fn main() {
    clearscreen::clear().unwrap();

    fn generate_status_string(status: bool) -> colorful::core::color_string::CString {
        match status {
            true => "Good".color(Color::Green1).bold(),
            false => "Bad".color(Color::Red3a).bold(),
        }
    }

    fn determine_time(duration: Duration) -> String {
        if duration.as_millis() == 0 {
            "".to_string()
        } else {
            let time = duration.as_millis() as f64;

            if time > 100.0 {
                format!("[Time: {}ms]", time.ceil().to_string().color(Color::Red3a))
            } else {
                format!(
                    "[Time: {}ms]",
                    time.ceil().to_string().color(Color::Purple1b)
                )
            }
        }
    }

    fn get_head_branch(branches: git2::Branches) -> String {

        let mut return_name: String = String::from("None"); 
        for branch in branches {

            let actual_branch = branch.unwrap();
            let branch_name = actual_branch.0.name().unwrap();
            
            match branch_name {
                Some(name) => {
                    return_name = name.to_string();
                },
                None => return_name = String::from("None")
            };
        };

        return return_name

    } 


    let mut global_dir = env::current_dir().unwrap();
    let mut time = SystemTime::now();
    let mut dir_string = global_dir.to_str().unwrap();
    let mut status = true;

    let repo = match Repository::open(global_dir.clone()) {
        Ok(repo) => repo,
        Err(e) => panic!("{}", e),
    };
    
    let branches = repo.branches(None).unwrap();

    print!(
        "[Working Dir: {}] [Status: {}] [Branch: {}] {}\n~> ",
        dir_string.color(Color::Purple1b),
        generate_status_string(status),
        get_head_branch(branches).color(Color::Purple1b),
        determine_time(time.elapsed().unwrap())
    );

    std::io::stdout().flush().unwrap();

    loop {
        let mut command_data = String::new();
        io::stdin().read_line(&mut command_data).unwrap();

        let split_command = command_data.split(' ').collect::<Vec<&str>>();

        #[cfg(target_os="windows")]        
        let split_command = split_command
            .iter()
            .map(|e| e.strip_suffix("\r\n").unwrap_or(e))
            .collect::<Vec<&str>>();
            
        #[cfg(target_os="linux")]
        let split_command = split_command
        .iter()
        .map(|e| e.strip_suffix("\n").unwrap_or(e))
        .collect::<Vec<&str>>();

        match split_command.get(0) {
            Some(command) => match command.to_lowercase().as_str() {
                "exit" => {
                    return;
                }
                "clear" => {
                    time = SystemTime::now();
                    clearscreen::clear().unwrap();
                    status = true;
                }
                "cls" => {
                    time = SystemTime::now();
                    clearscreen::clear().unwrap();
                    status = true;
                }
                "cd" => match split_command.get(1) {
                    Some(dir) => match env::set_current_dir(dir) {
                        Ok(_) => {
                            time = SystemTime::now();
                            global_dir = env::current_dir().unwrap();
                            dir_string = global_dir.to_str().unwrap();
                            status = true;
                        }
                        Err(_) => {
                            status = false;
                        }
                    },
                    None => {
                        println!("Couldn't get the directory to cd into...");
                        status = false;
                    }
                },
                _ => match command.to_lowercase().as_str() {
                    "cargo" => match Command::new(command).args(&split_command[1..]).spawn() {
                        Ok(mut output) => {
                            time = SystemTime::now();
                            output.wait().unwrap();
                            status = true;
                        }
                        Err(err) => {
                            println!("{err:?}", err = err);
                            status = false;
                        }
                    },
                    _ => {
                        println!("Unknwon...");
                        status = false;
                    }
                },
            },
            None => {
                println!(
                    "{} Failed to get commands from list of args...\n~> Args: {:?}",
                    "[ERROR]".color(Color::Red3b),
                    split_command
                );
                status = false;
            }
        }

        let branches = repo.branches(None).unwrap();
        print!(
            "[Working Dir: {}] [Status: {}] [Branch: {}] {}\n~> ",
            dir_string.color(Color::Purple1b),
            generate_status_string(status),
            get_head_branch(branches).color(Color::Purple1b),
            determine_time(time.elapsed().unwrap())
        );
    

        std::io::stdout().flush().unwrap();
    }
}
