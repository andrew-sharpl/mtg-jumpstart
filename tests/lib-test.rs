use jumpstart::*;
use jumpstart::Result;
use std::fs;
use std::path::PathBuf;

const ANGELS: &str = "tests/inputs/Angels1.txt";
const GOBLINS: &str = "tests/inputs/Goblins1.txt";
const ANGELS_GOBLINS: &str = "tests/expected/Angels1-Goblins1.txt";
const HELLO_PATH: &str = "tests/inputs/hello.txt";
const HELLO_TEXT: &str = "Hello, World!";

fn absolute_path(relative_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

#[test]
fn test_merge_small() {
    let a = String::from("a");
    let b = String::from("b");

    let expected = String::from("a\nb");
    let merged = merge(a, b);
    assert_eq!(expected, merged);
}

#[test]
fn test_merge_large() -> Result<()> {
    let angels_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ANGELS);
    let goblins_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOBLINS);
    let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ANGELS_GOBLINS);

    let angels_deck = fs::read_to_string(angels_path)?;
    let goblins_deck = fs::read_to_string(goblins_path)?;
    let expected = fs::read_to_string(expected_path)?;

    let merged = merge(angels_deck, goblins_deck);

    assert_eq!(expected, merged);
    Ok(())
}

#[test]
fn test_open() -> Result<()> {
    let hello_path = absolute_path(HELLO_PATH);
    let text = fs::read_to_string(hello_path)?;

    assert_eq!(text, "Hello, World!");
    Ok(())
}
