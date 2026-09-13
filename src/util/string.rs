pub trait Companyize {
    fn companyize(&self) -> String;
}

impl Companyize for String {
    fn companyize(&self) -> String {
        let mut chars: Vec<_> = self.chars().collect();
        let len = chars.len() - 1;

        if chars[len] == 'r'
            && chars[len - 1] == 'e'
            && !['a', 'e', 'i', 'o', 'u'].contains(&chars[len - 2])
        {
            chars.remove(len - 1);
        }

        chars.into_iter().collect()
    }
}

impl Companyize for &str {
    fn companyize(&self) -> String {
        self.to_string().companyize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_companyize() {
        assert_eq!("peer".to_string(), "peer".companyize());
        assert_eq!("pier".to_string(), "pier".companyize());
        assert_eq!("company".to_string(), "company".companyize());
        assert_eq!("leadswingr".to_string(), "leadswinger".companyize());
    }
}
