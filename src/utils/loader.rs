use crate::utils::types::AllTheThings;
use flate2::read::GzDecoder;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn load_things(file: &Path) -> anyhow::Result<AllTheThings> {
    let raw = fs::read_to_string(file)?;
    let all_the_things: AllTheThings = toml::from_str(&raw)?;
    info!("loaded all the things from {}", file.display());
    Ok(all_the_things)
}

pub fn load_words(file: &Path) -> io::Result<Vec<String>> {
    let file = fs::File::open(file)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);
    let words: Vec<String> = reader.lines().map_while(|l| l.ok()).collect();
    Ok(words)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;
    use std::path::PathBuf;

    #[test]
    fn test_load_things() {
        assert!(load_things(&fixture("test_things.toml")).is_ok());
        assert!(load_things(&PathBuf::from("/no/such/file")).is_err());
        assert!(load_things(&fixture("test_words.gz")).is_err());
    }

    #[test]
    fn test_load_words() {
        let words = load_words(&fixture("test_words.gz")).unwrap();
        assert_eq!(vec!["prognosticater"], words);

        assert!(load_words(&PathBuf::from("/no/such/file")).is_err());
    }
}
