use std::io;

mod note;
use note::NoteMgr;

fn main() {
    let mut note_mgr = NoteMgr::new();

    loop {
        println!("next command?");

        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(x) if x == 0 => println!("no command provided"),
            Ok(_) => {
                command.pop();
                handle_command(&command, &mut note_mgr);
            }
            Err(err) => println!("error on reading command: {err}"),
        }
    }
}

fn handle_command(command: &str, note_mgr: &mut NoteMgr) {
    match command {
        "add" => handle_add_note_command(note_mgr),
        "ls" => handle_list_notes_command(note_mgr),
        "rm" => handle_rm_note_command(note_mgr),
        "search" => handle_search_notes_command(note_mgr),
        _ => println!("boh"),
    }
}

fn handle_add_note_command(note_mgr: &mut NoteMgr) {
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
    note_mgr.add(String::from("a title"), new_note_content);
}

fn handle_list_notes_command(note_mgr: &mut NoteMgr) {
    for note in note_mgr.notes.iter() {
        println!("id -> {0}, content -> {1}", note.id, note.content);
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

fn handle_rm_note_command(note_mgr: &mut NoteMgr) {}
