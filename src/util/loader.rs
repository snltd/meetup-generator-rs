use super::types::AllTheThings;
use camino::Utf8Path;
use flate2::read::GzDecoder;
use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn load_things(file: &Utf8Path) -> anyhow::Result<AllTheThings> {
    let raw = fs::read_to_string(file)?;
    let all_the_things: AllTheThings = toml::from_str(&raw)?;
    tracing::info!("loaded all the things from {file}");

    Ok(all_the_things)
}

pub fn load_words(file: &Utf8Path) -> io::Result<Vec<String>> {
    let file = fs::File::open(file)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);
    let words: Vec<String> = reader.lines().map_while(|l| l.ok()).collect();

    Ok(words)
}

#[cfg(test)]
mod test {
    use super::*;
    use camino::Utf8PathBuf;
    use snltest::fixture;

    #[test]
    fn test_load_things() {
        assert!(load_things(&fixture!("test_things.toml")).is_ok());
        assert!(load_things(&Utf8PathBuf::from("/no/such/file")).is_err());
        assert!(load_things(&fixture!("test_words.gz")).is_err());
    }

    #[test]
    fn test_load_words() {
        let words = load_words(&fixture!("test_words.gz")).unwrap();
        assert_eq!(vec!["prognosticater"], words);
        assert!(load_words(&Utf8PathBuf::from("/no/such/file")).is_err());
    }
}
