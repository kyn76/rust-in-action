mod state;

use std::io::{self, Write};
use state::*;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut current_state: Box<dyn State> = Box::new(StartState);
    loop {
        println!("{}", current_state.message());
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        current_state = current_state.next(&input.trim());
        input.clear();
        if current_state.is_terminal() {
            println!("{}", current_state.message());
            break;
        }
    }
    Ok(())
}