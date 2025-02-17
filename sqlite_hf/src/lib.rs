use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to read the lyrics from the file and store them in a vector
pub fn read_lyrics() -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new("/workspaces/Rust-Zero-Shot-Classification/sqlite_hf/src/lyrics.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lyrics: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lyrics)
}

// Function to create a database and store the lyrics in it
pub fn create_db(lyrics: &[String]) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("lyrics.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS lyrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            line TEXT NOT NULL
        )",
        [],
    )?;

    let mut stmt = conn.prepare("INSERT INTO lyrics (line) VALUES (?1)")?;
    for line in lyrics {
        stmt.execute(params![line])?;
    }

    Ok(())
}

// Function to analyze the lyrics and return the word frequency
pub fn analyze_lyrics(lyrics: &[String]) -> HashMap<String, i32> {
    let mut word_freq = HashMap::new();
    for line in lyrics {
        for word in line.split_whitespace() {
            let word = word.to_lowercase();
            *word_freq.entry(word).or_insert(0) += 1;
        }
    }
    word_freq
}

// Function to print the word frequency
pub fn print_word_freq(word_freq: &HashMap<String, i32>) {
    for (word, freq) in word_freq.iter().take(100) {
        println!("{}: {}", word, freq);
    }
}
