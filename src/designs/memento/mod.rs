trait Memento<T> {
    fn restore(self) -> T;
    fn print(&self);
}

struct Document {
    content: String,
}

impl Document {
    pub fn new(content: &str) -> Self {
        Document {
            content: content.to_string(),
        }
    }

    pub fn save(&self) -> DocumentBackup {
        DocumentBackup {
            content: self.content.clone(),
        }
    }

    pub fn edit(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }
}

struct DocumentBackup {
    content: String,
}

impl Memento<Document> for DocumentBackup {
    fn restore(self) -> Document {
        Document {
            content: self.content,
        }
    }

    fn print(&self) {
        println!("Document backup: '{}'", self.content);
    }
}
#[test]
fn main() {
    let mut history = Vec::<DocumentBackup>::new();

    let mut document = Document::new("Initial content");

    document.edit("First edit");
    history.push(document.save());

    document.edit("Second edit");
    history.push(document.save());

    for backup in history.iter() {
        backup.print();
    }

    let document = history.pop().unwrap().restore();
    println!("Restored document: {}", document.content);

    let document = history.pop().unwrap().restore();
    println!("Restored document: {}", document.content);
}
