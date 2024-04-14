mod state;
mod inventory;

use std::io::{self, Write};
use state::{State, StartState};
use inventory::Inventory;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut inventory = Inventory::new();
    let mut current_state: Box<dyn State> = Box::new(StartState);
    loop {
        println!("{}", current_state.message(&inventory));
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        current_state = current_state.next(&input.trim(), &mut inventory);
        input.clear();
        if current_state.is_terminal() {
            println!("{}", current_state.message(&inventory));
            break;
        }
    }
    Ok(())
}