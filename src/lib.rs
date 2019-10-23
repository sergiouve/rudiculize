mod utils;

pub fn ridiculize(text: &str) -> String {
    let mut ridiculized = String::new();
    let mut iter = text.chars().peekable();

    while let Some(character) = iter.next() {
        let next_character: Option<&char> = iter.peek();

        if character == 'q' && *next_character.unwrap() == 'u' {
            continue;
        }

        if utils::is_vowel(&character) {
            let swapped_character = utils::swap_characters(&character, 'i');
            ridiculized.push_str(&swapped_character);
            continue;
        }

        // TODO
        if character == 'c' && *next_character.unwrap() != 'i' && utils::is_vowel(next_character.unwrap()) {
            let swapped_character = utils::swap_characters(&character, 'q');
            ridiculized.push_str(&swapped_character);
            ridiculized.push_str("u");
        }

        ridiculized.push(character);

        match next_character {
            Some(_) => continue,
            None => break,
        }
    }

    return ridiculized;
}
