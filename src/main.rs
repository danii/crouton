use clap::{App, SubCommand};
use crouton::crouton::Crouton;
use shadow_rs::shadow; // Add `Arg` back when I need it...

shadow!(build);

fn main() {
    let app = App::new(build::PROJECT_NAME)
        .about("A simple, small shell made in Rust.")
        .version(build::PKG_VERSION)
        .subcommand(SubCommand::with_name("build_info").about("Runs and shows build info."));

    let matches = app.get_matches();

    match matches.subcommand() {
        ("build_info", _) => {
            clearscreen::clear().unwrap();
            println!("Build info\n\n");

            let mut application = Crouton::new("{dir} {status} {time}");
            application.start();
        }

        _ => {
            clearscreen::clear().unwrap();
            let mut application = Crouton::new("{dir} {status} {time}");
            application.start();
        }
    }
}
