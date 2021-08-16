// use crouton::command; // Unused...
use crouton::crouton::Crouton;
use shadow_rs::shadow;

shadow!(build);

fn main() {
    clearscreen::clear().unwrap();
    let mut application = Crouton::new("{dir} {status} {time}");
    application.start();

    // let cmd = command::Command::new("owo",|a| match a {
    //     Some(args) => {
    //         for a in args {
    //             println!("Arg: {}", a);
    //         }
    //     }
    //     None => {}
    // });
    // println!(
    //     "Name: {}\n{:?}",
    //     cmd.name,
    //     (cmd.func)(Some(vec![
    //         String::from("owo"),
    //         String::from("ewe"),
    //         String::from("uwu"),
    //         String::from("pwp")
    //     ]))
    // );
}
