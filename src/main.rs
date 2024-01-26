use std::fs::read_to_string;

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
            distance: wagner_fischer(word, &correct_word),
            correct_word,
        })
        .collect();

    results.sort_by_key(|result| result.distance);
    results
}

fn wagner_fischer(s1: &str, s2: &str) -> usize {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let (s1, s2) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };

    let mut current_row: Vec<usize> = (0..s1.len() + 1).collect();
    for i in 1..s2.len() + 1 {
        let previous_row = current_row.clone();
        current_row = vec![0; s1.len() + 1];
        current_row[0] = i;
        for j in 1..current_row.len() {
            let (add, delete, change) = (
                previous_row[j] + 1,
                current_row[j - 1] + 1,
                previous_row[j - 1] + if s1[j - 1] != s2[i - 1] { 1 } else { 0 },
            );

            current_row[j] = *[add, delete, change].iter().min().unwrap();
        }
    }

    current_row[s1.len()]
}

fn load_dictionary(filename: &str) -> std::io::Result<Vec<String>> {
    Ok(read_to_string(filename)?
        .lines()
        .map(|line| line.trim().to_string())
        .collect())
}
