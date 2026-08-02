use std::{collections::HashMap, env, fs::File, io::Read};

fn main() {
    let words_count: &mut HashMap<String, u32> = &mut HashMap::new();

    let content: &mut String = &mut String::new();

    let _ = match &mut File::open("data/test1.txt") {
        Ok(src_file) => {src_file.read_to_string(content);
            count_words(content.chars().collect(), words_count);
            println!("{words_count:?}");
        }
        , // load all the file
        Err(_) => panic!("Error loading file"),
    };
}

fn count_words(content: Vec<char>, word_count: &mut HashMap<String, u32>) {
    // esto esta abstraido a comparacion del modelo hecho en el diagrama, usamos el String type para
    // no tener que preocuparnos de bastantes factores de un buffer inseguro
    let actual_word = &mut String::new();

    for c_idx in 0..content.len() - 1 {
        let c = content.get(c_idx).unwrap();

        if c.is_whitespace() || c.is_ascii_punctuation() {
            if word_count.contains_key(actual_word) {
                actual_word.clear();
                continue;
            }

            word_count.insert(actual_word.clone(), 1);

            actual_word.clear();
            continue;
        }

        actual_word.push(*c);

        if word_count.contains_key(actual_word) {
            word_count
                .entry(actual_word.clone())
                .and_modify(|count| *count += (1 as u32));
            continue;
        }
    }
}
