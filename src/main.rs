use std::{collections::HashMap, io};

fn main() {
    let mut notes: HashMap<String, String> = HashMap::new();

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

fn handle_command(command: &str, notes: &mut HashMap<String, String>) {
    match command {
        "add" => handle_add_note_command(notes),
        "ls" => handle_list_notes_command(notes),
        "help" => handle_help_command(),
        "rm" => handle_rm_note_command(notes),
        _ => println!("boh"),
    }
}

fn handle_add_note_command(notes: &mut HashMap<String, String>) {
    println!("write the note");

    let mut new_note_content = String::new();
    let mut content_ok = false;

    while !content_ok {
        match io::stdin().read_line(&mut new_note_content) {
            Ok(0) => {}
            Ok(_) => {
                content_ok = true;
            }
            Err(_) => {}
        }
    }

    new_note_content.pop();
    notes.insert(random_string::generate(6, "123456789"), new_note_content);
}

fn handle_list_notes_command(notes: &HashMap<String, String>) {
    for (id, content) in notes.iter() {
        println!("id -> {id}, content -> {content}");
    }
}

fn handle_help_command() {}

fn handle_rm_note_command(notes: &mut HashMap<String, String>) {}
