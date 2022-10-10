use crate::note::Note;

pub fn show_notes(notes: &[Note]) {
    for note in notes.iter() {
        println!(
            "id -> {0}, title -> {1}, content -> {2}",
            note.id, note.title, note.content
        );
    }
}
