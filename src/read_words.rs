use std::{fs::read_to_string, path::Path};

// words_alpha.txt from https://github.com/dwyl/english-words
const FILE_NAME: &str = "./words_alpha.txt";

pub fn read_words() -> Vec<String> {
    let path = Path::new(FILE_NAME);
    let file_contents = match read_to_string(&path) {
        Err(reason) => {
            panic!("Couldn't open {}: {}", FILE_NAME, reason);
        }
        Ok(file_contents) => file_contents,
    };
    file_contents
        .lines()
        .into_iter()
        .map(|line| line.to_string().to_uppercase())
        .filter(|word| word.len() == 5 || word.chars().count() == 5)
        .collect()
}
