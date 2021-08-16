use clap::{App, SubCommand};
use crouton::command;
use crouton::crouton::Crouton;
use shadow_rs::shadow; // Add `Arg` back when I need it...

shadow!(build);

fn main() {
    let app = App::new(build::PROJECT_NAME)
        .about("A simple, small shell made in Rust.")
        .version(build::PKG_VERSION)
        .subcommand(SubCommand::with_name("test").about("Tests the new command system..."));

    let matches = app.get_matches();

    match matches.subcommand() {
        ("test", _) => {
            #[allow(clippy::single_match)]
            let cmd = command::Command::new("owo", |a| match a {
                Some(args) => {
                    for a in args {
                        println!("Arg: {}", a);
                    }
                }
                None => {}
            });
            println!(
                "Name: {}\n{:?}",
                cmd.name,
                (cmd.func)(Some(vec![
                    String::from("owo"),
                    String::from("ewe"),
                    String::from("uwu"),
                    String::from("pwp")
                ]))
            );
        }
        _ => {
            clearscreen::clear().unwrap();
            let mut application = Crouton::new("{dir} {status} {time}");
            application.start();
        }
    }
}
