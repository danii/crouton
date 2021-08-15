use crouton::crouton::Crouton;
use shadow_rs::shadow;

shadow!(build);

fn main() {
    clearscreen::clear().unwrap();
    let mut application = Crouton::new("{dir} {status} {time}");
    application.start();
}
