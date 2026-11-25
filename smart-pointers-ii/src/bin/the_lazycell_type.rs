use std::cell::LazyCell;

fn main() {
    let content = "The quick brown fox jumps over the lazy dog";

    let word_count = LazyCell::new(|| {
        println!("Counting words...");
        content.split_whitespace().count()
    });

    println!("Words: {}", *word_count);
    // Counting words...
    // Words: 9

    // The cached value is returned
    println!("Words: {}", *word_count);
    // Words: 9
}
