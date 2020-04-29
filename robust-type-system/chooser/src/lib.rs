use std::{
    fmt,
    fmt::{Display, Formatter},
    io::{Error, ErrorKind},
};

use rand;
use rand::Rng;

#[derive(Debug)]
pub enum ChoiceType {
    Choice(&'static str),
    NoChoiceMade,
}

impl Display for ChoiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ChoiceType::Choice(c) => write!(f, "{}", c),
            ChoiceType::NoChoiceMade => write!(f, "No choice made"),
        }
    }
}

pub fn choose_random_color() -> Result<ChoiceType, std::io::Error> {
    let color_options = vec!["Blue", "Green", "Red"];

    let mut rng = rand::thread_rng();

    let make_choice = rng.gen_bool(0.9);
    let color_choice = rng.gen_range(0, color_options.len());

    return Ok(if make_choice {
        ChoiceType::Choice(color_options.get(color_choice).unwrap())
    } else {
        ChoiceType::NoChoiceMade
    });
}
