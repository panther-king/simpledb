use std::io::{stdin, stdout, Write};

const PROMPT: &str = "db> ";

enum MetaCommand {
    Exit,
    Success,
    Unrecognized,
}

enum Statement {
    Insert,
    Select,
}

fn do_meta_command(buffer: &str) -> MetaCommand {
    if buffer.trim() == ".exit" {
        return MetaCommand::Exit;
    }
    MetaCommand::Unrecognized
}

fn execute_statement(statement: &Statement) {
    match *statement {
        Statement::Insert => {
            println!("This is where we would do a insert.");
        }
        Statement::Select => {
            println!("This is where we would do a select.");
        }
    }
}

fn prepare_statement(buffer: &str) -> Option<Statement> {
    if buffer.trim().starts_with("insert") {
        return Some(Statement::Insert);
    }
    if buffer.trim().starts_with("select") {
        return Some(Statement::Select);
    }
    None
}

fn main() {
    let mut buffer = String::new();

    loop {
        print!("{}", PROMPT);

        let _ = stdout().flush();

        buffer.clear();
        stdin().read_line(&mut buffer).expect("fail");

        if buffer.starts_with(".") {
            match do_meta_command(&buffer) {
                MetaCommand::Exit => return,
                MetaCommand::Success => continue,
                MetaCommand::Unrecognized => {
                    println!("Unrecognized command {}", buffer);
                    continue;
                }
            }
        }

        let statement = prepare_statement(&buffer);
        if statement.is_none() {
            println!("Unrecognized keyword at start of {}", buffer);
            continue;
        }

        execute_statement(&statement.unwrap());
        println!("Executed.");
    }
}
