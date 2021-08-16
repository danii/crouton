#[derive(Debug)]
pub struct Command<F>
where
    F: Fn(Option<Vec<String>>),
{
    pub func: F,
    pub name: &'static str,
    pub takes_args: bool,
    pub max_args: Option<usize>,
    pub min_args: Option<usize>,
}

impl<F> Command<F>
where
    F: Fn(Option<Vec<String>>),
{
    pub fn new(name: &'static str, func: F) -> Self {
        Self {
            func,
            name,
            max_args: Some(0),
            min_args: Some(0),
            takes_args: true,
        }
    }
}
