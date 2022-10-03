use std::{collections::HashMap, io};

fn main() {
    let mut notes: HashMap<&str, &str> = HashMap::new();

    loop {
        println!("next command?");

        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(x) if x == 0 => println!("no command provided"),
            Ok(_) => {
                command.pop();
                handle_command(&command, &mut notes);
            }
            Err(err) => println!("error on reading command: {err}"),
        }
    }
}

fn handle_command(command: &str, notes: &mut HashMap<&str, &str>) {
    match command {
        "add" => handle_add_note_command(notes),
        "ls" => handle_list_notes_command(notes),
        "help" => handle_help_command(),
        "rm" => handle_rm_note_command(notes),
        _ => println!("boh"),
    }
}

fn handle_add_note_command(notes: &mut HashMap<&str, &str>) {}

fn handle_list_notes_command(notes: &HashMap<&str, &str>) {}

fn handle_help_command() {}

fn handle_rm_note_command(notes: &mut HashMap<&str, &str>) {}
