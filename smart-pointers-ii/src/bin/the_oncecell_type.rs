use std::cell::OnceCell;

struct TextDocument {
    content: String,
    cached_word_count: OnceCell<usize>,
}

impl TextDocument {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            cached_word_count: OnceCell::new(),
        }
    }

    fn word_count(&self) -> usize {
        *self.cached_word_count.get_or_init(|| {
            println!("Counting words...");
            self.content.split_whitespace().count()
        })
    }
}

fn main() {
    let doc = TextDocument::new("The quick brown fox jumps over the lazy dog");
    println!("Words: {}", doc.word_count());
    println!("Words: {}", doc.word_count());
    println!("Words: {}", doc.word_count());
    println!("Words: {}", doc.word_count());
    println!("Words: {}", doc.word_count());
}
