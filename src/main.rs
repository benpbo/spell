use std::{cmp::min, fs::read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _ = args.next();
    let dictionary_filename = args.next().unwrap();
    let dictionary = load_dictionary(&dictionary_filename)?;

    let word = args.next().unwrap();
    if dictionary.contains(&word) {
        println!("Word '{word}' contained in dictionary");
        return Ok(());
    }

    println!("Word '{word}' not found in dictionary");
    println!("Looking for similar words...");
    let suggestions = spell_check(&word, &dictionary);
    for suggestion in suggestions.iter().take(10) {
        println!("{} ({})", suggestion.correct_word, suggestion.distance);
    }

    Ok(())
}

struct SpellCheckSuggestion<'a> {
    distance: usize,
    correct_word: &'a str,
}

fn spell_check<'a>(word: &str, dictionary: &'a [String]) -> Vec<SpellCheckSuggestion<'a>> {
    let mut results: Vec<SpellCheckSuggestion> = dictionary
        .iter()
        .map(|correct_word| SpellCheckSuggestion {
            distance: wagner_fischer(word, correct_word),
            correct_word,
        })
        .collect();

    results.sort_by_key(|result| result.distance);
    results
}

fn wagner_fischer(word1: &str, word2: &str) -> usize {
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let (word1, word2) = if word1.len() > word2.len() {
        (word1, word2)
    } else {
        (word2, word1)
    };

    let mut current_row: Vec<usize> = (0..word1.len() + 1).collect();
    for (i, c) in word2.iter().enumerate() {
        let previous_row = current_row;
        current_row = vec![0; word1.len() + 1];
        current_row[0] = i + 1;
        for j in 1..current_row.len() {
            let add = previous_row[j] + 1;
            let delete = current_row[j - 1] + 1;
            let change = previous_row[j - 1] + if word1[j - 1] == *c { 0 } else { 1 };

            current_row[j] = min(min(add, delete), change);
        }
    }

    current_row[word1.len()]
}

fn load_dictionary(filename: &str) -> std::io::Result<Vec<String>> {
    Ok(read_to_string(filename)?
        .lines()
        .map(|line| line.trim().to_string())
        .collect())
}
