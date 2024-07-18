use std::{error::Error, path::PathBuf};
use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn merge(s1: String, s2: String) -> String {
    format!("{}\n{}", s1, s2)
}

pub fn read(filepath: &str) -> Result<()> {
   Ok(()) 
}

pub fn write(filepath: &str) -> Result<()> {
    Ok(())
}

pub fn generate_filename(path_a: &str, path_b: &str) -> Result<()> {
    Ok(()) 
}


pub fn select_decks(colors: &u8) -> Result<()> {
    Ok(())
}

pub fn run() -> Result<()> {
//    let args = get_args();
//    let (path_a, path_b) = select_decks(args.colors);
//    let deck_a = read(path_a);
//    let deck_b = read(path_b);
//    let new_deck = merge(deck_a, deck_b);
//    let new_file = generate_filename(path_a, path_b);
//    write(new_deck, new_file);
    Ok(())
}

