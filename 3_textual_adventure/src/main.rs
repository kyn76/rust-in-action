mod state;
mod inventory;
extern crate unicode_segmentation;

use std::io::{self, Write};
use state::{State, StartState};
use inventory::Inventory;
use std::thread;
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

const WRITING_DELAY: u64 = 10;


fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut inventory = Inventory::new();
    let mut current_state: Box<dyn State> = Box::new(StartState);
    loop {
        streamed_print(current_state.message(&inventory));
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        current_state = current_state.next(&input.trim(), &mut inventory);
        input.clear();
        if current_state.is_terminal() {
            streamed_print(current_state.message(&inventory));
            break;
        }
    }
    Ok(())
}


fn streamed_print(text: &str) {
    // Retrieve standard output
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // For each character (unicode grapheme) of the text
    for grapheme in text.graphemes(true) {
        // Write char in the standard output
        handle.write_all(grapheme.as_bytes()).unwrap();
        handle.flush().unwrap(); // Force writing in standard output

        // Wait a delay between each writing
        thread::sleep(Duration::from_millis(WRITING_DELAY));
    }
}