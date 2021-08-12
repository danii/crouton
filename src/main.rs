use colorful::{Color, Colorful};
use std::time::{Duration, SystemTime};
use std::{
    env,
    io::{self, Write},
    process::Command,
};

fn main() {
    println!("{esc}{esc}[2J{esc}[1;1H", esc = 27 as char);

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

    let mut global_dir = env::current_dir().unwrap();
    let mut time = SystemTime::now();
    let mut dir_string = global_dir.to_str().unwrap();
    let mut status = true;

    print!(
        "[Working Dir: {}] [Status: {}] {}\n~> ",
        dir_string.color(Color::Purple1b),
        generate_status_string(status),
        determine_time(time.elapsed().unwrap())
    );

    std::io::stdout().flush().unwrap();

    loop {
        let mut command_data = String::new();
        io::stdin().read_line(&mut command_data).unwrap();

        let split_command = command_data.split(' ').collect::<Vec<&str>>();
        let split_command = split_command
            .iter()
            .map(|e| e.strip_suffix("\r\n").unwrap_or(e))
            .collect::<Vec<&str>>();

        match split_command.get(0) {
            Some(command) => match command.to_lowercase().as_str() {
                "exit" => {
                    return;
                }
                "clear" => {
                    time = SystemTime::now();
                    println!("{esc}{esc}[2J{esc}[1;1H", esc = 27 as char);
                    status = true;
                }
                "cls" => {
                    time = SystemTime::now();
                    println!("{esc}{esc}[2J{esc}[1;1H", esc = 27 as char);
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

        print!(
            "[Working Dir: {}] [Status: {}] {}\n~> ",
            dir_string.color(Color::Purple1b),
            generate_status_string(status),
            determine_time(time.elapsed().unwrap())
        );

        std::io::stdout().flush().unwrap();
    }
}
