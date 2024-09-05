use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() {
    println!("Wortlänge: ");

    let word_len = get_word_len();

    println!("Wortlänge auf {} gesetzt.", word_len);

    let mut wordlist = get_wordlist(word_len);
    let mut choosen = Vec::<char>::new();

    // Gameloop
    loop {
        let most_common_letter = most_common_letter(wordlist.clone(), &choosen);
        choosen.push(most_common_letter);

        println!("Wähle: {}", most_common_letter);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Wortlänge ist weird");

        let input: Vec<usize> = input
            .split_whitespace()
            .map(|number| {
                number
                    .parse::<usize>()
                    .expect("eingabe konnte nicht zu zahl gemacht werden")
            })
            .collect();

        let old_wordlist_len = wordlist.len();

        // 0 für falscher Buchstabe
        if *input.first().expect("keine Eingabe?") == 0 {
            remove_words_by_letter(&mut wordlist, most_common_letter);
        }
        // 1 für richtiger Buchstabe
        else if *input.first().expect("keine Eingabe?") == 1 {
            let right_letters = input[1..].to_vec();
            retain_words_by_letter(&mut wordlist, most_common_letter, right_letters);
        }

        if wordlist.len() == 1 {
            println!("Das Wort ist: {}", wordlist.first().unwrap());
            break;
        } else if wordlist.is_empty() {
            println!("Ein Satz mit X");
            break;
        } else if wordlist.len() <= 10 {
            println!("Es gibt nur noch {} Wörter:", wordlist.len());
            for word in wordlist.iter() {
                println!("{}", word);
            }
        } else {
            println!(
                "es wurden {} Wörter eliminiert.",
                old_wordlist_len - wordlist.len()
            );
            println!("es bleiben noch {} Wörter.", wordlist.len());
        }
    }
}

fn get_word_len() -> usize {
    let mut word_len_str = String::new();

    io::stdin()
        .read_line(&mut word_len_str)
        .expect("Wortlänge ist weird");

    word_len_str
        .trim()
        .parse::<usize>()
        .expect("das kann man nicht parsen. Tja.")
}

fn get_wordlist(word_len: usize) -> Vec<String> {
    let file = File::open(format!("./wordlist-{}.txt", word_len))
        .expect("Bist du im richtigen Dir mit den wordlists?");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

// so gemacht, dass doppelte Buchstaben nur einmal mitgezählt werden für bessere Wahrscheinlichkeit
fn letter_counter(wordlist: Vec<String>) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for word in wordlist.iter() {
        let mut letters: HashSet<char> = HashSet::new();

        // set füllen
        for letter in word.chars() {
            letters.insert(letter);
        }

        // set in map eintragen
        for letter in letters.iter() {
            if let Some(val) = map.get_mut(letter) {
                *val += 1;
            } else {
                map.insert(*letter, 1);
            }
        }
    }

    map
}

fn most_common_letter(wordlist: Vec<String>, choosen_letters: &[char]) -> char {
    let letter_count = letter_counter(wordlist);
    let mut hashmap_array: Vec<_> = letter_count.into_iter().collect();
    hashmap_array.sort_by(|a, b| b.1.cmp(&a.1));
    let sorted_keys: Vec<_> = hashmap_array.into_iter().map(|(key, _value)| key).collect();

    for letter in sorted_keys {
        if !choosen_letters.contains(&letter) {
            return letter;
        }
    }
    panic!("WTF, man");
}

fn remove_words_by_letter(wordlist: &mut Vec<String>, letter: char) {
    wordlist.retain(|word| !word.contains(letter));
}

fn retain_words_by_letter(wordlist: &mut Vec<String>, letter: char, pos: Vec<usize>) {
    wordlist.retain(|word| {
        word.char_indices()
            .filter_map(|(index, c)| if c == letter { Some(index) } else { None })
            .collect::<Vec<usize>>()
            == pos
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_removing_by_letter() {
        let mut wordlist: Vec<String> = vec![
            "test".to_string(),
            "moin".to_string(),
            "lul".to_string(),
            "sesam".to_string(),
        ];
        remove_words_by_letter(&mut wordlist, 'e');
        assert_eq!(vec!["moin".to_string(), "lul".to_string()], wordlist)
    }

    #[test]
    fn retain_words() {
        let mut wordlist: Vec<String> = vec![
            "test".to_string(),
            "moin".to_string(),
            "lul".to_string(),
            "sesam".to_string(),
        ];
        retain_words_by_letter(&mut wordlist, 'e', vec![1]);
        assert_eq!(vec!["test".to_string(), "sesam".to_string()], wordlist)
    }
}
