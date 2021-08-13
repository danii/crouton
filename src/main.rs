use crouton::crouton::Crouton;

fn main() {
    clearscreen::clear().unwrap();
    let mut application = Crouton::new("{dir} {status} {time}");
    application.start();
}
