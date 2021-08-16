//use std::{env, fs:: {read_dir}, io::{self, Read, Write}, path::{Path, PathBuf}, process::Command, time::{Duration, SystemTime}};
use super::{command::Command as CCommand, git::statuses};
use colorful::{Color, Colorful};
use directories_next::UserDirs;
use git2::Repository;
use shell_words;
use std::{
    env,
    io::{self, BufRead, Write},
    path::PathBuf,
    process::{exit, Command},
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
    pub custom_cmds: Vec<CCommand>,
    pub user_dir: UserDirs,

    /* Config related fields */
    pub good_color: Color,
    pub bad_color: Color,
    pub branch_color: Color,
    pub branch_status_color: Color,
    pub path_color: Color,
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
            user_dir: UserDirs::new().unwrap(),
            currnent_time: SystemTime::now(),
            cmd_history: vec![],
            good_color: Color::MediumOrchid3,
            bad_color: Color::Red3a,
            branch_color: Color::Orange3,
            path_color: Color::Aquamarine3,
            branch_status_color: Color::Cyan3,
            custom_cmds: vec![CCommand::new("cd", |a, b| match b {
                Some(b) => match b.get(1) {
                    Some(dir) => match env::set_current_dir(dir) {
                        Ok(_) => {
                            a.currnent_time = SystemTime::now();
                            a.current_dir = env::current_dir().unwrap();
                            a.current_repo = a.get_current_repo();
                            a.current_head_branch = a.get_current_branch();

                            a.status = true;
                            a.cmd_history.push("cd".to_string());
                        }
                        Err(_) => {
                            a.status = false;
                        }
                    },
                    None => match env::set_current_dir(a.user_dir.home_dir()) {
                        Ok(_) => {
                            a.currnent_time = SystemTime::now();
                            a.current_dir = env::current_dir().unwrap();
                            a.current_repo = a.get_current_repo();
                            a.current_head_branch = a.get_current_branch();

                            a.status = true;
                            a.cmd_history.push("cd".to_string());
                        }
                        Err(_) => {
                            a.status = false;
                        }
                    },
                },
                None => {
                    a.status = false;
                }
            })],
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

    fn get_repo_status(&self) -> Option<statuses::RepoStatus> {
        let mut status = statuses::RepoStatus::default();

        match Command::new("git")
            .args(&[
                "-C",
                self.current_dir.to_str().unwrap(),
                "--no-optional-locks",
                "status",
                "--porcelain=2",
                "--branch",
            ])
            .output()
        {
            Ok(status) => status,
            Err(err) => panic!("{}", err),
        }
        .stdout
        .lines()
        .for_each(|s| match s {
            Ok(s) => {
                if !s.starts_with('#') {
                    status.add(&s)
                }
            }
            Err(err) => {
                println!("{}", err)
            }
        });

        Some(status)
    }

    fn display_branch(&self) -> String {
        match &self.current_head_branch {
            Some(branch) => format!(
                " [Branch: {branch}{status}] ",
                branch = branch.to_string().color(self.branch_color).bold(),
                status = match self.get_repo_status() {
                    Some(statuses) => {
                        let mut ret = String::from("");

                        if statuses.modified != 0 {
                            ret.push('!');
                        }

                        if statuses.deleted != 0 {
                            ret.push('-');
                        }

                        if statuses.staged != 0 {
                            ret.push('+');
                        }

                        if statuses.renamed != 0 {
                            ret.push('>')
                        }

                        format!("{}", ret.color(self.branch_status_color).bold())
                    }
                    None => String::from(""),
                }
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
            "[Working Dir: {dir}] [Status: {status}]{time}{branch}\n> ", //"[Working Dir: {}] [Status: {}] {}\n~> ",
            dir = self
                .current_dir
                .to_str()
                .unwrap_or("\"Failed to get dir\"")
                .color(self.path_color)
                .bold(),
            status = self.determine_status(self.status),
            branch = self.display_branch(),
            time = self.display_time()
        );

        std::io::stdout().flush().unwrap();
    }

    fn determine_status(&self, status: bool) -> colorful::core::color_string::CString {
        match status {
            true => "Good".color(self.good_color).bold(),
            false => "Bad".color(self.bad_color).bold(),
        }
    }

    pub fn start(&mut self) {
        self.current_repo = self.get_current_repo();
        self.current_head_branch = self.get_current_branch();
        self.print_header();

        loop {
            let mut command_data = String::new();
            io::stdin().read_line(&mut command_data).unwrap();

            let and_and_split = command_data.split("&&").collect::<Vec<&str>>();
            if and_and_split.len() > 1 {
                for section in and_and_split {
                    if !self.status {
                        break;
                    }

                    let split_command = shell_words::split(section).unwrap();

                    match split_command.get(0) {
                        Some(command) => self
                            .handle_command(command.to_lowercase().as_str(), split_command.clone()),
                        None => {
                            self.status = false;
                        }
                    }
                }
            } else {
                let semi_colon = command_data.split(';').collect::<Vec<&str>>();
                if semi_colon.len() > 1 {
                    for section in semi_colon {
                        let split_command = shell_words::split(section).unwrap();

                        match split_command.get(0) {
                            Some(command) => self.handle_command(
                                command.to_lowercase().as_str(),
                                split_command.clone(),
                            ),
                            None => {
                                self.status = false;
                            }
                        }
                    }
                } else {
                    let split_command = shell_words::split(&command_data).unwrap();

                    match split_command.get(0) {
                        Some(command) => self
                            .handle_command(command.to_lowercase().as_str(), split_command.clone()),
                        None => {
                            self.status = false;
                        }
                    }
                }
            }

            self.print_header()
        }
    }

    pub fn handle_command(&mut self, command: &str, split_command: Vec<String>) {
        match command {
            "exit" => {
                exit(0x0100);
            }
            "crouton_history" => match split_command.get(1) {
                Some(sub_cmd) => match sub_cmd.to_lowercase().as_str() {
                    "use" => {
                        let default_cmd = &String::from("crouton_history");
                        let mut command = self.cmd_history.get(0).unwrap_or(default_cmd);

                        if let Some(cmd) = split_command.get(2) {
                            command = match self.cmd_history.get(cmd.parse::<usize>().unwrap_or(0))
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
            _ => {
                let commands = &self
                    .custom_cmds
                    .iter()
                    .filter(|c| c.name == command.to_lowercase().as_str())
                    .collect::<Vec<&CCommand>>();

                match commands.get(0) {
                    Some(cmd) => {
                        (cmd.func)(self, Some((&split_command[0..]).to_vec()));
                    }
                    None => match Command::new(command).args(&split_command[1..]).spawn() {
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
                }
            }
        }
    }
}
