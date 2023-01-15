#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_unique_word() {
        let s = "this cat this rat this bat";
        let words: Vec<&str> = s.split_whitespace().collect();
        let mut unique_words = Vec::new();
        for word in words {
            if !unique_words.contains(&word) {
                unique_words.push(word);
            }
        }
        assert_eq!(unique_words.len(), 4);


    }
}
