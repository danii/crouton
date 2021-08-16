use clap::App;
use crouton::crouton::Crouton;
use shadow_rs::shadow; // Add `Arg` back when I need it...

shadow!(build);

fn main() {
    let app = App::new(build::PROJECT_NAME)
        .about("A simple, small shell made in Rust.")
        .version(build::PKG_VERSION);

    let matches = app.get_matches();

    matches.subcommand();
    {
        clearscreen::clear().unwrap();
        let mut application = Crouton::new("{dir} {status} {time}");
        application.start();
    }
}
