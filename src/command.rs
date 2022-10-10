use std::io;

pub enum Command {
    Add,
    Ls,
    Search,
}

pub enum CommandWithInput {
    Add { title: String, content: String },
}

fn ask_input(question: &str) -> String {
    let mut input = String::new();
    let mut got = false;

    while !got {
        println!("{}", question);
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop();
                got = true;
            }
            Err(error) => println!("error reading input - {error}"),
        }
    }

    input
}

pub fn ask_input_for_add_command() -> CommandWithInput {
    let title = ask_input("title?");
    let content = ask_input("content?");

    CommandWithInput::Add { title, content }
}

pub fn ask_command() -> Option<Command> {
    println!("next command?");

    let mut command = String::new();
    match io::stdin().read_line(&mut command) {
        Ok(_) => {
            command.pop();
            match command.as_str() {
                "add" => Some(Command::Add),
                "ls" => Some(Command::Ls),
                "search" => Some(Command::Search),
                _ => None,
            }
        }
        Err(error) => {
            println!("error while reading command - {error}");
            None
        }
    }
}
