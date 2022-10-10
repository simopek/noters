use std::io;

mod note;
use note::NoteMgr;

mod command;

mod ui;

fn main() {
    let mut note_mgr = NoteMgr::new();

    loop {
        match command::ask_command() {
            Some(command::Command::Add) => {
                let command::CommandWithInput::Add { title, content } =
                    command::ask_input_for_add_command();
                note_mgr.add(title, content);
            }
            Some(command::Command::Ls) => {
                ui::show_notes(&note_mgr.notes);
            }
            _ => {}
        }
    }
}

fn handle_search_notes_command(note_mgr: &mut NoteMgr) {
    println!("enter the search key");

    let mut search_key = String::new();
    let mut search_key_ok = false;

    while !search_key_ok {
        match io::stdin().read_line(&mut search_key) {
            Ok(0) => {}
            Ok(_) => {
                search_key_ok = true;
            }
            Err(_) => {}
        }
    }

    search_key.pop();
    for note in note_mgr.search(&search_key).iter() {
        println!("id -> {0}, content -> {1}", note.id, note.content);
    }
}
