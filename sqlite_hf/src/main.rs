use sqlite_hf::{analyze_lyrics, create_db, print_word_freq, read_lyrics};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lyrics = read_lyrics()?; // Read lyrics from file
    create_db(&lyrics)?; // Store lyrics in the database

    let word_freq = analyze_lyrics(&lyrics); // Analyze word frequency
    print_word_freq(&word_freq); // Print results

    Ok(())
}
