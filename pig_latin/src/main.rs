fn main() {
    let mut text: String =
        String::from("I have to go to the university now but I will come back soon.");
    const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut new_text = String::new();
    let vowel_insert = String::from("hay");
    let consonant_insert = String::from("ay");
    let mut temp_word = String::new();
    let mut first_words_charter = ' ';
    let mut is_first_words_charter = true;
    for symbol in text.chars() {
        if symbol == ' ' {
            if VOWELS.contains(&first_words_charter.to_lowercase().last().unwrap()) {
                new_text.push_str(&temp_word);
                new_text.push('-');
                new_text.push_str(&vowel_insert);
            } else {
                new_text.push_str(&temp_word);
                new_text.push('-');
                new_text.push(first_words_charter);
                new_text.push_str(&consonant_insert);
            }

            new_text.push(symbol);
            temp_word.clear();
            is_first_words_charter = true;
            continue;
        }

        if is_first_words_charter {
            first_words_charter = symbol;
            if VOWELS.contains(&first_words_charter.to_lowercase().last().unwrap()) {
                temp_word.push(first_words_charter);
            }

            is_first_words_charter = false;
            continue;
        }

        temp_word.push(symbol);
    }

    println!("{}", new_text);
}
