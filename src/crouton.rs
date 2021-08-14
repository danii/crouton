//use std::{env, fs:: {read_dir}, io::{self, Read, Write}, path::{Path, PathBuf}, process::Command, time::{Duration, SystemTime}};
use colorful::{Color, Colorful};
use git2::Repository;
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::Command,
    time::{Duration, SystemTime},
};

pub struct Crouton {
    pub header_string: &'static str,
    pub current_dir: PathBuf,
    pub status: bool,
    pub current_repo: Option<Repository>,
    pub current_head_branch: Option<String>,
    pub currnent_time: SystemTime,
    pub cmd_history: Vec<String>,
}

impl Crouton {
    pub fn new(header_string: &'static str) -> Crouton {
        let path = env::current_dir().unwrap();

        Crouton {
            header_string,
            current_dir: path,
            status: true,
            current_repo: None,
            current_head_branch: None,
            currnent_time: SystemTime::now(),
            cmd_history: vec![],
        }
    }

    fn get_current_repo(&self) -> Option<Repository> {
        match Repository::open(self.current_dir.to_str().unwrap()) {
            Ok(repo) => Some(repo),
            Err(_) => None,
        }
    }

    fn get_current_branch(&self) -> Option<String> {
        let head = match &self.current_repo {
            Some(repo) => match repo.head() {
                Ok(referance) => Some(referance),
                Err(_) => None,
            },
            None => None,
        };

        match head {
            Some(head) => {
                let shorthand = head.shorthand();
                shorthand.map(std::string::ToString::to_string)
            }
            None => None,
        }
    }

    fn display_branch(&self) -> String {
        match &self.current_head_branch {
            Some(branch) => format!(
                " [Branch: {branch}] ",
                branch = branch.to_string().color(Color::MediumPurple3a).bold()
            ),
            None => String::from(""),
        }
    }

    fn display_time(&self) -> String {
        let duration = self
            .currnent_time
            .elapsed()
            .unwrap_or_else(|_| Duration::new(1, 1));

        if duration.as_millis() == 0 {
            "".to_string()
        } else {
            let time = duration.as_millis() as f64;

            if time > 100.0 {
                format!(
                    " [Time: {}ms] ",
                    time.ceil().to_string().color(Color::MediumVioletRed).bold()
                )
            } else {
                format!(
                    " [Time: {}ms] ",
                    time.ceil()
                        .to_string()
                        .color(Color::MediumSpringGreen)
                        .bold()
                )
            }
        }
    }

    fn print_header(&self) {
        print!(
            "[Working Dir: {dir}] [Status: {status}]{time}{branch}\n❯ ", //"[Working Dir: {}] [Status: {}] {}\n~> ",
            dir = self
                .current_dir
                .to_str()
                .unwrap_or("\"Failed to get dir\"")
                .color(Color::MediumOrchid3)
                .bold(),
            status = self.determine_status(self.status),
            branch = self.display_branch(),
            time = self.display_time()
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
        self.current_repo = self.get_current_repo();
        self.current_head_branch = self.get_current_branch();
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
                .map(|e| e.strip_suffix('\n').unwrap_or(e))
                .collect::<Vec<&str>>();

            match split_command.get(0) {
                Some(command) => match command.to_lowercase().as_str() {
                    "exit" => {
                        return;
                    }
                    "crouton_history" => match split_command.get(1) {
                        Some(sub_cmd) => match sub_cmd.to_lowercase().as_str() {
                            "use" => {
                                let default_cmd = &String::from("crouton_history");
                                let mut command = self.cmd_history.get(0).unwrap_or(default_cmd);

                                if let Some(cmd) = split_command.get(2) {
                                    command = match self
                                        .cmd_history
                                        .get(cmd.parse::<usize>().unwrap_or(0))
                                    {
                                        Some(cmd) => cmd,
                                        None => command,
                                    }
                                }

                                match Command::new(command).args(&split_command[3..]).spawn() {
                                    Ok(mut output) => {
                                        self.currnent_time = SystemTime::now();
                                        output.wait().unwrap();
                                        self.status = true;
                                    }
                                    Err(err) => {
                                        println!("{err:?}", err = err);
                                        self.status = false;
                                    }
                                }
                            }
                            _ => {
                                println!("unknown subcommand for crouton_history.");
                                self.status = false;
                            }
                        },
                        None => {
                            println!("History: {:?}", self.cmd_history);
                            self.status = true;
                        }
                    },
                    "cd" => match split_command.get(1) {
                        // Handling the CD command our selves in needed so that I can
                        // set the branch if the directory is a git repo. I can't
                        // think of any other way to do it :(
                        Some(dir) => match env::set_current_dir(dir) {
                            Ok(_) => {
                                self.currnent_time = SystemTime::now();
                                self.current_dir = env::current_dir().unwrap();
                                self.current_repo = self.get_current_repo();
                                self.current_head_branch = self.get_current_branch();

                                self.status = true;
                                self.cmd_history.push("cd".to_string());
                            }
                            Err(_) => {
                                self.status = false;
                            }
                        },
                        None => {
                            #[cfg(target_os = "windows")]
                            let root_path = match std::env::var("SystemDrive") {
                                    Ok(path) => {
                                        format!("{}", path).as_str()
                                    }
                                    Err(_) => "C:\\",
                            };

                            #[cfg(target_os = "linux")]
                            let root_path = "/";

                            match env::set_current_dir(root_path) {
                                Ok(_) => {
                                    self.currnent_time = SystemTime::now();
                                    self.current_dir = env::current_dir().unwrap();
                                    self.current_repo = self.get_current_repo();
                                    self.current_head_branch = self.get_current_branch();

                                    self.status = true;
                                    self.cmd_history.push("cd".to_string());
                                }
                                Err(_) => {
                                    self.status = false;
                                }
                            }
                        }
                    },
                    _ => match Command::new(command).args(&split_command[1..]).spawn() {
                        Ok(mut output) => {
                            self.currnent_time = SystemTime::now();
                            output.wait().unwrap();

                            self.current_dir = env::current_dir().unwrap();
                            self.current_repo = self.get_current_repo();
                            self.current_head_branch = self.get_current_branch();

                            self.cmd_history.push(command.to_string());
                            self.status = true;
                        }
                        Err(err) => {
                            println!("{err:?}", err = err);
                            self.cmd_history.push(command.to_string());
                            self.status = false;
                        }
                    },
                },
                None => {
                    self.status = false;
                }
            }

            self.print_header()
        }
    }
}
