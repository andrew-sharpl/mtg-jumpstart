use jumpstart::*;
use jumpstart::Result;
use std::fs;
use std::path::PathBuf;

const ANGELS: &str = "tests/inputs/Angels1.txt";
const GOBLINS: &str = "tests/inputs/Goblins1.txt";
const ANGELS_GOBLINS: &str = "tests/expected/Angels1-Goblins1.txt";

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
fn test_deck_file() -> Result<()> {
    let angels_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ANGELS);
    let goblins_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOBLINS);

    let destination_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/expected/Goblins1-Angels1.txt");
    if destination_path.exists() {
        fs::remove_file(&destination_path)?;
    }

    let _ = create_new_deck(&angels_path, &goblins_path, &destination_path);
    assert!(destination_path.exists());

    let angels_deck = fs::read_to_string(angels_path)?;
    let goblins_deck = fs::read_to_string(goblins_path)?;
    let result_deck = fs::read_to_string(destination_path)?;

    let merged = merge(goblins_deck, angels_deck);
    assert_eq!(merged, result_deck);

    Ok(())
}
