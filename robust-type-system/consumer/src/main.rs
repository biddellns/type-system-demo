use chooser::{choose_random_color, ChoiceType};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let choice = choose_random_color()?;
    match choice {
        ChoiceType::NoChoiceMade => println!("No choice made"),
        ChoiceType::Choice(c) => println!("{} was chosen!!", c),
    }

    // Expanded Form

    // let choice = choose_random_color();
    // match choice {
    //     Ok(v) => {
    //         match v {
    //             ChoiceType::NoChoiceMade => println!("No choice made!"),
    //             ChoiceType::Choice(c) => println!("{} was chosen!!", c),
    //         }
    //     }
    //     Err(e) => eprintln!("Error while choosing color: {}", e)
    // }

    Ok(())
}
