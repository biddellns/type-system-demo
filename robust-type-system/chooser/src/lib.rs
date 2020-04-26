use rand;
use rand::Rng;
use std::{
    io::{Error, ErrorKind}
};

pub enum ChooseResult<'a> {
    Choice(&'a str),
    NoChoiceMade,
}

pub fn choose(email: &str) -> Result<ChooseResult, std::io::Error> {
    let color_options = vec!("Blue", "Green", "Red");

    let mut rng = rand::thread_rng();


    let make_choice = rng.gen_bool(3.0);
    let color_choice = rng.gen_range(0, color_options.len());

    Err(Error::new(ErrorKind::Other, "oops"))
}