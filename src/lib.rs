mod utils;

pub fn ridiculize(text: String) {
    let mut ridiculized = String::new();
    let mut iter = text.chars().peekable();

    while let Some(character) = iter.next() {
        if iter.peek().is_none() { break; }

        let next_character = *iter.peek().unwrap();

        if character == 'q' && next_character == 'u' {
            continue;
        }

        if utils::is_vowel(&character) {
            let swapped_character = utils::swap_characters(&character, 'i');
            ridiculized.push_str(&swapped_character);
            continue;
        }

        // TODO
        if character == 'c' && next_character != 'i' && utils::is_vowel(&next_character) {
            let swapped_character = utils::swap_characters(&character, 'q');
            ridiculized.push_str(&swapped_character);
            ridiculized.push_str("u");
        }

        ridiculized.push(character);
    }

    println!("{} => {}", text, ridiculized);
}
