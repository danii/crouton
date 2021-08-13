//use std::{env, fs:: {read_dir}, io::{self, Read, Write}, path::{Path, PathBuf}, process::Command, time::{Duration, SystemTime}};
use colorful::{Color, Colorful};
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

#[derive(Debug)]
pub struct Crouton {
    pub header_string: &'static str,
    pub current_dir: PathBuf,
    pub status: bool,
}

impl Crouton {
    pub fn new(header_string: &'static str) -> Crouton {
        Crouton {
            header_string,
            current_dir: env::current_dir().unwrap(),
            status: true,
        }
    }

    fn print_header(&self) {
        print!(
            "[Working Dir: {dir}] [Status: {status}]\n❯ ", //"[Working Dir: {}] [Status: {}] {}\n~> ",
            dir = self
                .current_dir
                .to_str()
                .unwrap_or("\"Failed to get dir\"")
                .color(Color::MediumPurple3a)
                .bold(),
            status = self.determine_status(self.status),
        );

        std::io::stdout().flush().unwrap();
    }

    fn determine_status(&self, status: bool) -> colorful::core::color_string::CString {
        match status {
            true => "Good".color(Color::MediumSpringGreen).bold(),
            false => "Bad".color(Color::MediumVioletRed).bold(),
        }
    }

    pub fn start(&mut self) {
        self.print_header();

        loop {
            let mut command_data = String::new();
            io::stdin().read_line(&mut command_data).unwrap();

            let split_command = command_data.split(' ').collect::<Vec<&str>>();

            #[cfg(target_os = "windows")]
            let split_command = split_command
                .iter()
                .map(|e| e.strip_suffix("\r\n").unwrap_or(e))
                .collect::<Vec<&str>>();

            #[cfg(target_os = "linux")]
            let split_command = split_command
                .iter()
                .map(|e| e.strip_suffix("\n").unwrap_or(e))
                .collect::<Vec<&str>>();

            match split_command.get(0) {
                Some(command) => match Command::new(command).args(&split_command[1..]).spawn() {
                    Ok(mut output) => {
                        output.wait().unwrap();
                        self.status = true;
                    }
                    Err(err) => {
                        println!("{err:?}", err = err);
                        self.status = false;
                    }
                },
                None => {
                    self.status = false;
                }
            }

            self.print_header()
        }
    }
}
