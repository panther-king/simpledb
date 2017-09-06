use std::io::{stdin, stdout, Write};

const PROMPT: &str = "db> ";

fn main() {
    let mut buffer = String::new();

    loop {
        print!("{}", PROMPT);

        let _ = stdout().flush();

        buffer.clear();
        stdin().read_line(&mut buffer).expect("fail");

        if buffer.trim() == ".quit" {
            break;
        } else {
            println!("Unrecognized command '{}'", buffer.trim());
        }
    }
}
