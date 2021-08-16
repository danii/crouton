use super::crouton::Crouton;

pub struct Command {
    //<F>
    // where
    //     F: Fn(Option<Vec<String>>),
    // {
    pub func: fn(&mut Crouton, Option<Vec<String>>),
    pub name: &'static str,
    pub takes_args: bool,
    pub max_args: Option<usize>,
    pub min_args: Option<usize>,
}

// impl<F> Command<F>
// where
//     F: Fn(Option<Vec<String>>),
// {
impl Command {
    pub fn new(name: &'static str, func: fn(&mut Crouton, Option<Vec<String>>)) -> Self {
        Self {
            func,
            name,
            max_args: Some(0),
            min_args: Some(0),
            takes_args: true,
        }
    }
}
