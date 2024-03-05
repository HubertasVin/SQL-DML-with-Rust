use rustyline::{error::ReadlineError, DefaultEditor};

use crate::executor::execute_statement;

mod data_frame;
mod database;
mod parser;
mod statement_adt;
mod executor;
mod table_drawer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.as_str() == "exit" {
                    break;
                }

                let parsed_statement = parser::parse_to_adt(line.as_str());
                match parsed_statement {
                    Ok(statement) => {
                        match execute_statement(statement) {
                            Ok(data_frame) => {
                                table_drawer::draw_table(data_frame);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
                let _ = rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    let _ = rl.save_history("history.txt");
    Ok(())
}
