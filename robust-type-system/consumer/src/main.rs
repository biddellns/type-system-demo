use chooser::{choose_random_color, ChoiceType};

fn main() {
   let choice = choose_random_color();
    match choice {
        Ok(v) => println!("Result: {}", v),
        Err(e) => eprintln!("Got an error: {}", e)
    }
}
