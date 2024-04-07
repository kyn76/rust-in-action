use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("Enter some text");
    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    println!("You typed: {}", input.trim());
    Ok(())
}