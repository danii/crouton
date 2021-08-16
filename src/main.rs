use clap::{App, SubCommand};
use colorful::Colorful;
use crouton::colors;
use crouton::crouton::Crouton;
use shadow_rs::shadow;

shadow!(build);

fn main() {
    let app = App::new(build::PROJECT_NAME)
        .about("A simple, small shell made in Rust.")
        .version(build::PKG_VERSION)
        .subcommand(SubCommand::with_name("build_info").about("Runs and shows build info."))
        .subcommand(
            SubCommand::with_name("show_colors")
                .about("Displays all colors that are supported in config files."),
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        ("build_info", _) => {
            clearscreen::clear().unwrap();
            println!("Build info\n\n");

            let mut application = Crouton::new("{dir} {status} {time}");
            application.start();
        }

        ("show_colors", _) => {
            println!("The following is all colors that can be used in Crouton Configs.");

            let colors = vec![
                "Red",
                "Red1",
                "Red3a",
                "Red3b",
                "Purple3",
                "Purple4a",
                "Purple4b",
                "MediumPurple",
                "MediumPurple1",
                "MediumPurple2a",
                "MediumPurple2b",
                "MediumPurple3a",
                "MediumPurple3b",
                "MediumPurple4",
                "Cyan",
                "Cyan1",
                "Cyan2",
                "Cyan3",
                "Blue",
                "Blue1",
                "Blue3a",
                "Blue3b",
                "DarkBlue",
                "NavyBlue",
                "RosyBrown",
                "Yellow",
                "Yellow1",
                "Yellow2",
                "Yellow3a",
                "Yellow3b",
                "Yellow4a",
                "Yellow4b",
                "GreenYellow",
                "LightYellow",
                "LightYellow3",
                "Orange1",
                "Orange3",
                "DarkOrange3a",
                "DarkOrange3b",
                "OrangeRed1",
                "Orange4a",
                "Orange4b",
            ];

            for color in colors {
                println!("I am {}", color.color(colors::color_from_str(color)))
            }
        }

        _ => {
            clearscreen::clear().unwrap();
            let mut application = Crouton::new("{dir} {status} {time}");
            application.start();
        }
    }
}
