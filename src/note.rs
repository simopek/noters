pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
}

pub struct NoteMgr {
    pub notes: Vec<Note>,
}

impl NoteMgr {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn add(&mut self, title: String, content: String) {
        self.notes.push(Note {
            id: random_string::generate(6, "123456789"),
            title,
            content,
        })
    }

    pub fn search(&self, search_key: &str) -> Vec<&Note> {
        self.notes
            .iter()
            .filter(|note| note.title.contains(search_key) || note.content.contains(search_key))
            .collect()
    }
}
