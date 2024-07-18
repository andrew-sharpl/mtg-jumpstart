use std::{error::Error, path::PathBuf};
use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn merge(s1: String, s2: String) -> String {
    format!("{}\n{}", s1, s2)
}

pub fn read(filepath: &str) -> Result<String> {}

pub fn write(filepath: &str) -> Result<()> {}

pub fn generate_filename(path_a: &str, path_b: &str) -> Result<String> {}

pub fn select_decks(colors: &u8) -> Result<PathBuf> {}

pub fn run() {
    let args = get_args();
    let (path_a, path_b) = select_decks(args.colors);
    let deck_a = read(path_a);
    let deck_b = read(path_b);
    let new_deck = merge(deck_a, deck_b);
    let new_file = generate_filename(path_a, path_b);
    write(new_deck, new_file);
}
//pub fn create_new_deck(path_a: &PathBuf, path_b: &PathBuf, destination: &PathBuf) -> Result<()> {
    //let deck_a = fs::read_to_string(path_a)?;
    //let deck_b = fs::read_to_string(path_b)?;
    //let deck_a_name = path_a.file_stem().unwrap().to_str().unwrap();
    //let deck_b_name = path_b.file_stem().unwrap().to_str().unwrap();
    
    
    //Ok(())
//}
//


