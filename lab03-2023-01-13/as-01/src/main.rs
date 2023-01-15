fn main(){
    let s = "this cat this rat this bat";
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut unique_words = Vec::new();
    for word in words {
        if !unique_words.contains(&word) {
            unique_words.push(word);
        }
    }
    println!("unique words: {:?}", unique_words);

}